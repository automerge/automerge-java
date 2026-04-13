use am::transaction::Transactable;
use automerge::{self as am, transaction::OwnedTransaction};
use automerge_jni_macros::jni_fn;
use jni::{
    errors::ThrowRuntimeExAndDefault,
    objects::{JClass, JObject},
    signature::RuntimeMethodSignature,
    strings::JNIStr,
};

use crate::{
    interop::{changehash_to_jobject, JavaPointer},
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
    type Output<'a>;
    unsafe fn execute<'b, T: Transactable>(
        self,
        env: &jni::Env<'b>,
        tx: &mut T,
    ) -> Result<Self::Output<'b>, jni::errors::Error>;
}

unsafe fn do_tx_op<'local, Op: TransactionOp>(
    env: &mut jni::Env<'local>,
    tx_pointer: JObject<'local>,
    op: Op,
) -> Result<Op::Output<'local>, jni::errors::Error> {
    let mut tx = am::transaction::OwnedTransaction::borrow_from_pointer(env, tx_pointer)?;
    op.execute(env, &mut *tx)
}

pub(crate) const COMMITRESULT_CLASS: &JNIStr = am_classname!("CommitResult");

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn commitTransaction<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    doc_obj: JObject<'local>,
) -> JObject<'local> {
    env.with_env(|env| {
        let tx = OwnedTransaction::take_from_pointer(env, tx)?;
        let (doc, hash, patches) = tx.commit();

        doc.return_to_pointer(env, doc_obj)?;

        let hash_jobject = hash
            .map(|h| {
                let hash_jobj = changehash_to_jobject(env, &h)?;
                make_optional(env, (&hash_jobj).into())
            })
            .unwrap_or_else(|| make_empty_option(env))?;
        let patches_jobject = patches.store_as_pointer(env)?;

        // TODO: figure out how to replace this with jni_sig!
        let constructor_sig = RuntimeMethodSignature::from_str(format!(
            "(Ljava/util/Optional;L{};)V",
            am::PatchLog::POINTER_CLASS
        ))?;
        env.new_object(
            COMMITRESULT_CLASS,
            constructor_sig.method_signature(),
            &[(&hash_jobject).into(), (&patches_jobject).into()],
        )
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn rollbackTransaction<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    doc_obj: JObject<'local>,
) {
    env.with_env(|env| {
        let tx = OwnedTransaction::take_from_pointer(env, tx)?;
        let (doc, _) = tx.rollback();
        doc.return_to_pointer(env, doc_obj)?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}
