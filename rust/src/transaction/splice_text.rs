use jni::{
    objects::{JClass, JString},
    sys::jlong,
    NativeMethod,
};

use crate::{interop::unwrap_or_throw_amg_exc, obj_id::JavaObjId};

use super::{do_tx_op, TransactionOp};

struct SpliceTextOp<'a> {
    obj: JavaObjId,
    idx: jlong,
    delete: jlong,
    value: JString<'a>,
}

impl<'a> TransactionOp for SpliceTextOp<'a> {
    type Output<'local> = ();

    unsafe fn execute<'local, T: automerge::transaction::Transactable>(
        self,
        env: &jni::Env<'local>,
        tx: &mut T,
    ) -> Result<Self::Output<'local>, jni::errors::Error> {
        unwrap_or_throw_amg_exc(
            env,
            tx.splice_text(
                self.obj,
                self.idx as usize,
                self.delete as isize,
                &self.value.to_string(),
            ),
        )
    }
}

use crate::bindings;

const _METHODS: &[NativeMethod] = &[
    ams_native! { static extern fn splice_text(tx: bindings::TransactionPointer, obj: bindings::ObjectId, start_idx: jlong, delete_count: jlong, chars: JString) },
];

fn splice_text<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    start_idx: jlong,
    delete_count: jlong,
    chars: JString<'local>,
) -> jni::errors::Result<()> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    unsafe {
        do_tx_op(
            env,
            tx.into(),
            SpliceTextOp {
                obj,
                idx: start_idx,
                delete: delete_count,
                value: chars,
            },
        )
    }
}
