use am::transaction::Transactable;
use automerge::{self as am, transaction::OwnedTransaction};
use jni::{
    objects::{JClass, JObject},
    NativeMethod,
};

use crate::interop::{changehash_to_jobject, JavaPointer};

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

use crate::bindings;

const _METHODS: &[NativeMethod] = &[
    ams_native! { static extern fn commit_transaction(tx: bindings::TransactionPointer, doc: bindings::DocPointer) -> bindings::CommitResult },
    ams_native! { static extern fn rollback_transaction(tx: bindings::TransactionPointer, doc: bindings::DocPointer) },
];

fn commit_transaction<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    doc: bindings::DocPointer<'local>,
) -> jni::errors::Result<bindings::CommitResult<'local>> {
    let tx = unsafe { OwnedTransaction::take_from_pointer(env, tx)? };
    let (doc_am, hash, patches) = tx.commit();

    unsafe { doc_am.return_to_pointer(env, doc)? };

    let hash_opt = match hash {
        Some(h) => {
            let jhash = changehash_to_jobject(env, &h)?;
            let jhash_obj: JObject = jhash.into();
            bindings::Optional::of(env, &jhash_obj)?
        }
        None => bindings::Optional::empty(env)?,
    };
    let patch_log = unsafe { patches.store_as_pointer(env)? };

    bindings::CommitResult::new(env, &hash_opt, &patch_log)
}

fn rollback_transaction<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    doc: bindings::DocPointer<'local>,
) -> jni::errors::Result<()> {
    let tx = unsafe { OwnedTransaction::take_from_pointer(env, tx)? };
    let (doc_am, _) = tx.rollback();
    unsafe { doc_am.return_to_pointer(env, doc)? };
    Ok(())
}
