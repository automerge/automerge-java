use automerge::transaction::Transactable;
use automerge_jni_macros::jni_fn;
use jni::{
    errors::ThrowRuntimeExAndDefault,
    objects::{JClass, JObject, JString},
    sys::jlong,
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

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn deleteInMap<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj: JObject<'local>,
    key: JString<'local>,
) {
    env.with_env(|env| {
        let k: String = key.to_string();
        let obj = JavaObjId::from_jobject(env, obj)?;
        do_tx_op(env, tx_pointer, DeleteOp { obj, key: k.into() })
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn deleteInList<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj: JObject<'local>,
    idx: jlong,
) {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj)?;
        let idx = read_usize(env, idx)?;
        do_tx_op(
            env,
            tx_pointer,
            DeleteOp {
                obj,
                key: idx.into(),
            },
        )?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}
