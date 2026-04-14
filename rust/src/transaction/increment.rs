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

struct IncrementOp {
    obj: JavaObjId,
    key: automerge::Prop,
    value: i64,
}

impl TransactionOp for IncrementOp {
    type Output<'local> = ();

    unsafe fn execute<'local, T: automerge::transaction::Transactable>(
        self,
        env: &jni::Env<'local>,
        tx: &mut T,
    ) -> Result<Self::Output<'local>, jni::errors::Error> {
        unwrap_or_throw_amg_exc(env, tx.increment(self.obj, self.key, self.value))
    }
}

use crate::bindings;

const _METHODS: &[NativeMethod] = &[
    ams_native! { static extern fn increment_in_map(tx: bindings::TransactionPointer, obj: bindings::ObjectId, key: JString, value: jlong) },
    ams_native! { static extern fn increment_in_list(tx: bindings::TransactionPointer, obj: bindings::ObjectId, idx: jlong, value: jlong) },
];

fn increment_in_map<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    key: JString<'local>,
    value: jlong,
) -> jni::errors::Result<()> {
    let key: String = key.to_string();
    let obj = JavaObjId::from_object_id(env, obj)?;
    unsafe {
        do_tx_op(
            env,
            tx.into(),
            IncrementOp {
                obj,
                key: key.into(),
                value,
            },
        )
    }
}

fn increment_in_list<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    idx: jlong,
    value: jlong,
) -> jni::errors::Result<()> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    let idx = read_usize(env, idx)?;
    unsafe {
        do_tx_op(
            env,
            tx.into(),
            IncrementOp {
                obj,
                key: idx.into(),
                value,
            },
        )
    }
}
