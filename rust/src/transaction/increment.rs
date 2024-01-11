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

struct IncrementOp {
    obj: jobject,
    key: automerge::Prop,
    value: i64,
}

impl TransactionOp for IncrementOp {
    type Output = ();

    unsafe fn execute<T: automerge::transaction::Transactable>(
        self,
        env: jni::JNIEnv,
        tx: &mut T,
    ) -> Self::Output {
        let obj = obj_id_or_throw!(&env, self.obj, ());
        match tx.increment(obj, self.key, self.value) {
            Ok(_) => {}
            Err(e) => {
                env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
            }
        }
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn incrementInMap(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    key: JString,
    value: jlong,
) {
    let key: String = env.get_string(key).unwrap().into();
    do_tx_op(
        env,
        tx_pointer,
        IncrementOp {
            obj: obj_pointer,
            key: key.into(),
            value,
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn incrementInList(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    idx: jlong,
    value: jlong,
) {
    let idx = match usize::try_from(idx) {
        Ok(i) => i,
        Err(_) => {
            env.throw_new(AUTOMERGE_EXCEPTION, "index cannot be negative")
                .unwrap();
            return;
        }
    };
    do_tx_op(
        env,
        tx_pointer,
        IncrementOp {
            obj: obj_pointer,
            key: idx.into(),
            value,
        },
    )
}
