use automerge_jni_macros::jni_fn;
use jni::{
    objects::JString,
    sys::{jlong, jobject},
};

use crate::{obj_id::JavaObjId, AUTOMERGE_EXCEPTION};

use super::{do_tx_op, TransactionOp};

struct SpliceTextOp<'a> {
    obj: jobject,
    idx: jlong,
    delete: jlong,
    value: JString<'a>,
}

impl<'a> TransactionOp for SpliceTextOp<'a> {
    type Output = ();

    unsafe fn execute<T: automerge::transaction::Transactable>(
        self,
        env: jni::JNIEnv,
        tx: &mut T,
    ) -> Self::Output {
        let obj = JavaObjId::from_raw(&env, self.obj).unwrap();
        let value: String = env.get_string(self.value).unwrap().into();
        match tx.splice_text(obj, self.idx as usize, self.delete as isize, &value) {
            Ok(_) => {}
            Err(e) => {
                env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
            }
        }
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn spliceText(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    start_idx: jlong,
    delete_count: jlong,
    chars: JString,
) {
    do_tx_op(
        env,
        tx_pointer,
        SpliceTextOp {
            obj: obj_pointer,
            idx: start_idx,
            delete: delete_count,
            value: chars,
        },
    )
}
