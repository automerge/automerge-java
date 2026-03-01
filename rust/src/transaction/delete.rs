use automerge::transaction::Transactable;
use automerge_jni_macros::jni_fn;
use jni::{
    objects::JString,
    sys::{jlong, jobject, jstring},
};

use crate::{
    interop::throw_amg_exc_or_fatal,
    obj_id::{obj_id_or_throw, JavaObjId},
};

use super::{do_tx_op, TransactionOp};

struct DeleteOp {
    obj: jobject,
    key: automerge::Prop,
}

impl TransactionOp for DeleteOp {
    type Output = ();

    unsafe fn execute<T: Transactable>(self, env: &mut jni::JNIEnv, tx: &mut T) -> Self::Output {
        let obj = obj_id_or_throw!(env, self.obj, ());
        match tx.delete(obj, self.key) {
            Ok(_) => {}
            Err(e) => {
                throw_amg_exc_or_fatal(env, e.to_string());
            }
        }
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn deleteInMap(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    key: jstring,
) {
    let key = JString::from_raw(key);
    let k: String = env.get_string(&key).unwrap().into();
    do_tx_op(
        &mut env,
        tx_pointer,
        DeleteOp {
            obj: obj_pointer,
            key: k.into(),
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn deleteInList<'local>(
    mut env: jni::JNIEnv<'local>,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    idx: jlong,
) {
    let idx = match usize::try_from(idx) {
        Ok(idx) => idx,
        Err(_) => {
            throw_amg_exc_or_fatal(&mut env, "Index out of bounds");
            return;
        }
    };
    do_tx_op(
        &mut env,
        tx_pointer,
        DeleteOp {
            obj: obj_pointer,
            key: idx.into(),
        },
    );
}
