use am::transaction::Transactable;
use automerge as am;
use automerge_jni_macros::jni_fn;
use jni::{objects::JObject, sys::jobject};

use crate::{
    interop::{changehash_to_jobject, AsPointerObj},
    java_option::{make_empty_option, make_optional},
};

mod delete;
mod increment;
mod insert;
mod mark;
mod set;
mod splice;
mod splice_text;

trait TransactionOp {
    type Output;
    unsafe fn execute<T: Transactable>(self, env: jni::JNIEnv, tx: &mut T) -> Self::Output;
}

trait OwnedTransactionOp {
    type Output;
    unsafe fn execute(self, env: jni::JNIEnv, tx: am::transaction::Transaction) -> Self::Output;
}

unsafe fn do_tx_op<Op: TransactionOp>(env: jni::JNIEnv, tx_pointer: jobject, op: Op) -> Op::Output {
    let tx = am::transaction::Transaction::from_pointer_obj(&env, tx_pointer).unwrap();
    op.execute(env, tx)
}

unsafe fn do_owned_tx_op<Op: OwnedTransactionOp>(
    env: jni::JNIEnv,
    tx_pointer: jobject,
    op: Op,
) -> Op::Output {
    let tx = am::transaction::Transaction::owned_from_pointer_obj(&env, tx_pointer).unwrap();
    op.execute(env, *tx)
}

struct Commit;

pub(crate) const COMMITRESULT_CLASS: &str = am_classname!("CommitResult");

impl OwnedTransactionOp for Commit {
    type Output = jobject;

    unsafe fn execute(self, env: jni::JNIEnv, tx: am::transaction::Transaction) -> Self::Output {
        let (hash, patches) = tx.commit();
        let hash_jobject = hash
            .map(|h| {
                let hash_jobj = changehash_to_jobject(&env, &h)?;
                make_optional(&env, hash_jobj.into())
            })
            .unwrap_or_else(|| make_empty_option(&env))
            .unwrap();
        let patches_jobject = JObject::from_raw(patches.to_pointer_obj(&env).unwrap());
        let commit_result = env
            .new_object(
                COMMITRESULT_CLASS,
                format!("(Ljava/util/Optional;L{};)V", am::PatchLog::classname()),
                &[hash_jobject.into(), patches_jobject.into()],
            )
            .unwrap();
        commit_result.into_raw()
    }
}
#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn commitTransaction(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jobject,
) -> jobject {
    do_owned_tx_op(env, tx_pointer, Commit)
}

struct Rollback;

impl OwnedTransactionOp for Rollback {
    type Output = ();

    unsafe fn execute(self, _env: jni::JNIEnv, tx: am::transaction::Transaction) -> Self::Output {
        tx.rollback();
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn rollbackTransaction(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jobject,
) {
    do_owned_tx_op(env, tx_pointer, Rollback);
}

#[derive(Debug, thiserror::Error)]
enum OpError {
    #[error(transparent)]
    Automerge(#[from] am::AutomergeError),
    #[error(transparent)]
    Jni(#[from] jni::errors::Error),
}
