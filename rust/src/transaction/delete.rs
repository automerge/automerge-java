use automerge::transaction::Transactable;
use automerge_jni_macros::jni_fn;
use jni::{
    objects::JString,
    sys::{jlong, jobject},
};

use crate::{
    obj_id::{obj_id_or_throw, JavaObjId},
    AUTOMERGE_EXCEPTION,
};

use super::{do_tx_op, TransactionOp};

struct DeleteOp {
    obj: jobject,
    key: automerge::Prop,
}

impl TransactionOp for DeleteOp {
    type Output = ();

    unsafe fn execute<T: Transactable>(self, env: jni::JNIEnv, tx: &mut T) -> Self::Output {
        let obj = obj_id_or_throw!(&env, self.obj, ());
        match tx.delete(obj, self.key) {
            Ok(_) => {}
            Err(e) => {
                env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
            }
        }
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn deleteInMap(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    key: JString,
) {
    let k: String = env.get_string(key).unwrap().into();
    do_tx_op(
        env,
        tx_pointer,
        DeleteOp {
            obj: obj_pointer,
            key: k.into(),
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn deleteInList(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    idx: jlong,
) {
    let idx = match usize::try_from(idx) {
        Ok(idx) => idx,
        Err(_) => {
            env.throw_new(AUTOMERGE_EXCEPTION, "Index out of bounds")
                .unwrap();
            return;
        }
    };
    do_tx_op(
        env,
        tx_pointer,
        DeleteOp {
            obj: obj_pointer,
            key: idx.into(),
        },
    )
}
