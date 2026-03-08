use am::transaction::Transactable;
use automerge::{self as am, transaction::OwnedTransaction};
use automerge_jni_macros::jni_fn;
use jni::sys::jobject;

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
    unsafe fn execute<'a, 'b, T: Transactable>(
        self,
        env: &'a mut jni::JNIEnv<'b>,
        tx: &mut T,
    ) -> Self::Output;
}

unsafe fn do_tx_op<Op: TransactionOp>(
    env: &mut jni::JNIEnv,
    tx_pointer: jobject,
    op: Op,
) -> Op::Output {
    let tx = am::transaction::OwnedTransaction::from_pointer_obj(env, tx_pointer).unwrap();
    op.execute(env, tx)
}

pub(crate) const COMMITRESULT_CLASS: &str = am_classname!("CommitResult");

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn commitTransaction(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jobject,
    doc_pointer: jobject,
) -> jobject {
    let tx = OwnedTransaction::owned_from_pointer_obj(&mut env, tx_pointer).unwrap();
    let (doc, hash, patches) = tx.commit();
    doc.set_pointer(&mut env, doc_pointer).unwrap();
    
    let hash_jobject = hash
        .map(|h| {
            let hash_jobj = changehash_to_jobject(&mut env, &h)?;
            make_optional(&mut env, (&hash_jobj).into())
        })
        .unwrap_or_else(|| make_empty_option(&mut env))
        .unwrap();
    let patches_jobject = patches.to_pointer_obj(&mut env).unwrap();
    let commit_result = env
        .new_object(
            COMMITRESULT_CLASS,
            format!("(Ljava/util/Optional;L{};)V", am::PatchLog::classname()),
            &[(&hash_jobject).into(), (&patches_jobject).into()],
        )
        .unwrap();
    commit_result.into_raw()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn rollbackTransaction(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jobject,
    doc_pointer: jobject,
) {
    let tx = OwnedTransaction::owned_from_pointer_obj(&mut env, tx_pointer).unwrap();
    let (doc, _) = tx.rollback();
    doc.set_pointer(&mut env, doc_pointer).unwrap();
}
