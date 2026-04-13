use am::ObjType;
use automerge as am;
use automerge_jni_macros::jni_fn;
use jni::{
    errors::ThrowRuntimeExAndDefault,
    jni_sig, jni_str,
    objects::{JByteArray, JClass, JObject, JString},
    sys::{jboolean, jdouble, jlong},
};

use crate::{
    interop::{read_u64, read_usize, unwrap_or_throw_amg_exc},
    obj_id::JavaObjId,
    obj_type::JavaObjType,
};

use super::{do_tx_op, TransactionOp};

struct InsertOp<V> {
    obj: JavaObjId,
    index: jlong,
    value: V,
}

impl TransactionOp for InsertOp<am::ScalarValue> {
    type Output<'b> = ();
    unsafe fn execute<'a, T: am::transaction::Transactable>(
        self,
        env: &jni::Env<'a>,
        tx: &mut T,
    ) -> Result<Self::Output<'a>, jni::errors::Error> {
        let idx = read_usize(env, self.index)?;
        unwrap_or_throw_amg_exc(env, tx.insert(self.obj, idx, self.value))
    }
}

impl TransactionOp for InsertOp<ObjType> {
    type Output<'b> = JavaObjId;

    unsafe fn execute<'a, T: am::transaction::Transactable>(
        self,
        env: &jni::Env<'a>,
        tx: &mut T,
    ) -> Result<Self::Output<'a>, jni::errors::Error> {
        let idx = read_usize(env, self.index)?;
        let value = unwrap_or_throw_amg_exc(env, tx.insert_object(self.obj, idx, self.value))?;
        Ok(JavaObjId::from(value))
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn insertDoubleInList<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj_pointer: JObject<'local>,
    idx: jlong,
    value: jdouble,
) {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj_pointer)?;
        do_tx_op(
            env,
            tx_pointer,
            InsertOp {
                obj,
                index: idx,
                value: am::ScalarValue::F64(value),
            },
        )
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn insertStringInList<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj_pointer: JObject<'local>,
    idx: jlong,
    value: JString<'local>,
) {
    let value: String = value.to_string();
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj_pointer)?;
        do_tx_op(
            env,
            tx_pointer,
            InsertOp {
                obj,
                index: idx,
                value: am::ScalarValue::Str(value.into()),
            },
        )?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn insertIntInList<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj_pointer: JObject<'local>,
    idx: jlong,
    value: jlong,
) {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj_pointer)?;
        do_tx_op(
            env,
            tx_pointer,
            InsertOp {
                obj,
                index: idx,
                value: am::ScalarValue::Int(value),
            },
        )?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn insertUintInList<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj_pointer: JObject<'local>,
    idx: jlong,
    value: jlong,
) {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj_pointer)?;
        let int = read_u64(env, value)?;
        do_tx_op(
            env,
            tx_pointer,
            InsertOp {
                obj,
                index: idx,
                value: am::ScalarValue::Uint(int),
            },
        )?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn insertBytesInList<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj: JObject<'local>,
    idx: jlong,
    value: JByteArray<'local>,
) {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj)?;
        let bytes = env.convert_byte_array(&value)?;
        do_tx_op(
            env,
            tx_pointer,
            InsertOp {
                obj,
                index: idx,
                value: am::ScalarValue::Bytes(bytes),
            },
        )?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn insertNullInList<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj: JObject<'local>,
    idx: jlong,
) {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj)?;
        do_tx_op(
            env,
            tx_pointer,
            InsertOp {
                obj,
                index: idx,
                value: am::ScalarValue::Null,
            },
        )?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn insertCounterInList<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj_pointer: JObject<'local>,
    idx: jlong,
    value: jlong,
) {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj_pointer)?;
        do_tx_op(
            env,
            tx_pointer,
            InsertOp {
                obj,
                index: idx,
                value: am::ScalarValue::counter(value),
            },
        )?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn insertDateInList<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj: JObject<'local>,
    idx: jlong,
    date: JObject<'local>,
) {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj)?;
        let date_millis = env
            .call_method(date, jni_str!("getTime"), jni_sig!("()J"), &[])?
            .j()?;
        do_tx_op(
            env,
            tx_pointer,
            InsertOp {
                obj,
                index: idx,
                value: am::ScalarValue::Timestamp(date_millis),
            },
        )?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn insertBoolInList<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj: JObject<'local>,
    idx: jlong,
    value: jboolean,
) {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj)?;
        do_tx_op(
            env,
            tx_pointer,
            InsertOp {
                obj,
                index: idx,
                value: am::ScalarValue::Boolean(value),
            },
        )?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn insertObjectInList<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj_pointer: JObject<'local>,
    idx: jlong,
    value: JObject<'local>,
) -> JObject<'local> {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj_pointer)?;
        let obj_type = JavaObjType::from_java_enum(env, value)?;
        let result = do_tx_op(
            env,
            tx_pointer,
            InsertOp {
                obj,
                index: idx,
                value: am::ObjType::from(obj_type),
            },
        )?;
        result.into_jobject(env)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}
