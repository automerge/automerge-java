use am::op_observer::HasPatches;
use am::transaction::{Observed, UnObserved};
use am::VecOpObserver;
use automerge as am;
use automerge::transaction::{Transactable, Transaction as AmTransaction};
use automerge_jni_macros::jni_fn;
use jni::{objects::JObject, sys::jobject};

use crate::interop::{AsPointerObj, ToJniObject};
use crate::java_option::make_optional;
use crate::make_empty_option;

mod delete;
mod increment;
mod insert;
mod mark;
mod set;
mod splice;
mod splice_text;

trait Transaction: Transactable {
    type Output: ToJniObject;
    fn commit(self) -> Option<Self::Output>;
    fn rollback(self);
}

impl<'a> Transaction for am::transaction::Transaction<'a, Observed<VecOpObserver>> {
    type Output = (am::ChangeHash, Vec<am::Patch<char>>);

    fn commit(self) -> Option<Self::Output> {
        let (mut o, c) = am::transaction::Transaction::commit(self);
        c.map(|c| (c, o.take_patches()))
    }

    fn rollback(self) {
        am::transaction::Transaction::rollback(self);
    }
}

impl<'a> Transaction for am::transaction::Transaction<'a, UnObserved> {
    type Output = am::ChangeHash;
    fn commit(self) -> Option<am::ChangeHash> {
        am::transaction::Transaction::commit(self)
    }

    fn rollback(self) {
        am::transaction::Transaction::rollback(self);
    }
}

trait TransactionOp {
    type Output;
    unsafe fn execute<T: Transaction>(self, env: jni::JNIEnv, tx: &mut T) -> Self::Output;
}

trait OwnedTransactionOp {
    type Output;
    unsafe fn execute<T: Transaction>(self, env: jni::JNIEnv, tx: T) -> Self::Output;
}

unsafe fn do_tx_op<Op: TransactionOp>(env: jni::JNIEnv, tx_pointer: jobject, op: Op) -> Op::Output {
    let jtx = JObject::from_raw(tx_pointer);
    let is_observed = env
        .is_instance_of(jtx, AmTransaction::<Observed<VecOpObserver>>::classname())
        .unwrap();

    if is_observed {
        let tx = AmTransaction::<'_, Observed<VecOpObserver>>::from_pointer_obj(&env, tx_pointer)
            .unwrap();
        op.execute(env, tx)
    } else {
        let tx = AmTransaction::<'_, UnObserved>::from_pointer_obj(&env, tx_pointer).unwrap();
        op.execute(env, tx)
    }
}

unsafe fn do_owned_tx_op<Op: OwnedTransactionOp>(
    env: jni::JNIEnv,
    tx_pointer: jobject,
    op: Op,
) -> Op::Output {
    let jtx = JObject::from_raw(tx_pointer);
    let is_observed = env
        .is_instance_of(jtx, AmTransaction::<Observed<VecOpObserver>>::classname())
        .unwrap();

    if is_observed {
        let tx =
            AmTransaction::<'_, Observed<VecOpObserver>>::owned_from_pointer_obj(&env, tx_pointer)
                .unwrap();
        op.execute(env, *tx)
    } else {
        let tx = AmTransaction::<'_, UnObserved>::owned_from_pointer_obj(&env, tx_pointer).unwrap();
        op.execute(env, *tx)
    }
}

struct Commit;

impl OwnedTransactionOp for Commit {
    type Output = jobject;

    unsafe fn execute<T: Transaction>(self, env: jni::JNIEnv, tx: T) -> Self::Output {
        match tx.commit() {
            Some(h) => {
                let obj = h.to_jni_object(&env).unwrap();
                make_optional(&env, obj.into()).unwrap().into_raw()
                //let obj = match h.to_jni_object(&env) {
                //Ok(o) => o,
                //Err(e) => {
                //env.exception_describe().unwrap();
                //JObject::null()
                //}
                //};
                //match make_optional(&env, obj.into()) {
                //Ok(o) => o.into_raw(),
                //Err(_) => {
                //env.exception_describe().unwrap();
                //JObject::null().into_raw()
                //}
                //}
            }
            None => make_empty_option(&env).unwrap().into_raw(),
        }
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn commitObservedTransaction(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jobject,
) -> jobject {
    do_owned_tx_op(env, tx_pointer, Commit)
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn commitUnobservedTransaction(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jobject,
) -> jobject {
    do_owned_tx_op(env, tx_pointer, Commit)
}

struct Rollback;

impl OwnedTransactionOp for Rollback {
    type Output = ();

    unsafe fn execute<T: Transaction>(self, _env: jni::JNIEnv, tx: T) {
        tx.rollback()
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
