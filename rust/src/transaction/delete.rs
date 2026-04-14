use automerge::transaction::Transactable;
use jni::{
    objects::{JClass, JString},
    sys::jlong,
    NativeMethod,
};

use crate::{
    interop::{read_usize, unwrap_or_throw_amg_exc},
    obj_id::JavaObjId,
};

use super::{do_tx_op, TransactionOp};

struct DeleteOp {
    obj: JavaObjId,
    key: automerge::Prop,
}

impl TransactionOp for DeleteOp {
    type Output<'a> = ();

    unsafe fn execute<'a, T: Transactable>(
        self,
        env: &jni::Env<'a>,
        tx: &mut T,
    ) -> Result<Self::Output<'a>, jni::errors::Error> {
        unwrap_or_throw_amg_exc(env, tx.delete(self.obj, self.key))
    }
}

use crate::bindings;

const _METHODS: &[NativeMethod] = &[
    ams_native! { static extern fn delete_in_map(tx: bindings::TransactionPointer, obj: bindings::ObjectId, key: JString) },
    ams_native! { static extern fn delete_in_list(tx: bindings::TransactionPointer, obj: bindings::ObjectId, idx: jlong) },
];

fn delete_in_map<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    key: JString<'local>,
) -> jni::errors::Result<()> {
    let k: String = key.to_string();
    let obj = JavaObjId::from_object_id(env, obj)?;
    unsafe { do_tx_op(env, tx.into(), DeleteOp { obj, key: k.into() }) }
}

fn delete_in_list<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    idx: jlong,
) -> jni::errors::Result<()> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    let idx = read_usize(env, idx)?;
    unsafe {
        do_tx_op(
            env,
            tx.into(),
            DeleteOp {
                obj,
                key: idx.into(),
            },
        )
    }
}
