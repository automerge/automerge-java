use automerge_jni_macros::jni_fn;
use jni::{
    errors::ThrowRuntimeExAndDefault,
    objects::{JClass, JObject, JString},
    sys::jlong,
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

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn spliceText<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    start_idx: jlong,
    delete_count: jlong,
    chars: JString<'local>,
) {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj)?;
        do_tx_op(
            env,
            tx,
            SpliceTextOp {
                obj,
                idx: start_idx,
                delete: delete_count,
                value: chars,
            },
        )
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}
