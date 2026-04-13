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

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn incrementInMap<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj: JObject<'local>,
    key: JString<'local>,
    value: jlong,
) {
    let key: String = key.to_string();
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj)?;
        do_tx_op(
            env,
            tx_pointer,
            IncrementOp {
                obj,
                key: key.into(),
                value,
            },
        )
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn incrementInList<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj_pointer: JObject<'local>,
    idx: jlong,
    value: jlong,
) {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj_pointer)?;
        let idx = read_usize(env, idx)?;
        do_tx_op(
            env,
            tx_pointer,
            IncrementOp {
                obj,
                key: idx.into(),
                value,
            },
        )?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}
