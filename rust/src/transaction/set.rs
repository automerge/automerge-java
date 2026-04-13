use automerge as am;
use automerge::transaction::Transactable;
use automerge_jni_macros::jni_fn;
use jni::errors::ThrowRuntimeExAndDefault;
use jni::objects::{JByteArray, JClass};
use jni::sys::jboolean;
use jni::{jni_sig, jni_str};
use jni::{
    objects::{JObject, JString},
    sys::{jint, jlong},
};

use crate::interop::{read_usize, unwrap_or_throw_amg_exc};
use crate::obj_id::JavaObjId;
use crate::obj_type::JavaObjType;
use crate::prop::JProp;

use super::{do_tx_op, TransactionOp};

struct SetOp<'a, V: Into<automerge::ScalarValue>> {
    obj: JavaObjId,
    prop: JProp<'a>,
    value: V,
}

impl<'a, V: Into<automerge::ScalarValue>> TransactionOp for SetOp<'a, V> {
    type Output<'local> = ();

    unsafe fn execute<'local, T: Transactable>(
        self,
        env: &jni::Env<'local>,
        tx: &mut T,
    ) -> Result<Self::Output<'local>, jni::errors::Error> {
        let key = self.prop.try_into_prop(env)?;

        unwrap_or_throw_amg_exc(env, tx.put(self.obj, key, self.value))
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setDoubleInMap<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj: JObject<'local>,
    key: JString<'local>,
    value: jni::sys::jdouble,
) {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj)?;
        do_tx_op(
            env,
            tx_pointer,
            SetOp {
                obj,
                prop: key.into(),
                value,
            },
        )
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setBytesInMap<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj: JObject<'local>,
    key: JString<'local>,
    value: JByteArray<'local>,
) {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj)?;
        let bytes = env.convert_byte_array(value)?;
        do_tx_op(
            env,
            tx_pointer,
            SetOp {
                obj,
                prop: key.into(),
                value: bytes.as_slice().to_vec(),
            },
        )
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setStringInMap<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj: JObject<'local>,
    key: JString<'local>,
    value: JString<'local>,
) {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj)?;
        do_tx_op(
            env,
            tx_pointer,
            SetOp {
                obj,
                prop: key.into(),
                value: value.to_string(),
            },
        )
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setIntInMap<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj: JObject<'local>,
    key: JString<'local>,
    value: jint,
) {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj)?;
        do_tx_op(
            env,
            tx_pointer,
            SetOp {
                obj,
                prop: key.into(),
                value: value as i64,
            },
        )
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setUintInMap<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj: JObject<'local>,
    key: JString<'local>,
    value: jint,
) {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj)?;
        do_tx_op(
            env,
            tx_pointer,
            SetOp {
                obj,
                prop: key.into(),
                value: am::ScalarValue::Uint(value as u64),
            },
        )
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setBoolInMap<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj: JObject<'local>,
    key: JString<'local>,
    value: jboolean,
) {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj)?;
        do_tx_op(
            env,
            tx_pointer,
            SetOp {
                obj,
                prop: key.into(),
                value: am::ScalarValue::Boolean(value),
            },
        )?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setNullInMap<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj: JObject<'local>,
    key: JString<'local>,
) {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj)?;
        do_tx_op(
            env,
            tx_pointer,
            SetOp {
                obj,
                prop: key.into(),
                value: am::ScalarValue::Null,
            },
        )?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setCounterInMap<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj: JObject<'local>,
    key: JString<'local>,
    value: jlong,
) {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj)?;
        do_tx_op(
            env,
            tx_pointer,
            SetOp {
                obj,
                prop: key.into(),
                value: am::ScalarValue::counter(value),
            },
        )?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setDateInMap<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj_pointer: JObject<'local>,
    key: JString<'local>,
    date: JObject<'local>,
) {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj_pointer)?;
        let date_millis = env
            .call_method(date, jni_str!("getTime"), jni_sig!("()J"), &[])?
            .j()?;
        do_tx_op(
            env,
            tx_pointer,
            SetOp {
                obj,
                prop: key.into(),
                value: am::ScalarValue::Timestamp(date_millis),
            },
        )?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setDoubleInList<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj: JObject<'local>,
    idx: jlong,
    value: jni::sys::jdouble,
) {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj)?;
        do_tx_op(
            env,
            tx_pointer,
            SetOp {
                obj,
                prop: idx.into(),
                value,
            },
        )?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setIntInList<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj: JObject<'local>,
    idx: jlong,
    value: jint,
) {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj)?;
        do_tx_op(
            env,
            tx_pointer,
            SetOp {
                obj,
                prop: idx.into(),
                value: value as i64,
            },
        )?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setUintInList<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj: JObject<'local>,
    idx: jlong,
    value: jint,
) {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj)?;
        do_tx_op(
            env,
            tx_pointer,
            SetOp {
                obj,
                prop: idx.into(),
                value: am::ScalarValue::Uint(value as u64),
            },
        )?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setStringInList<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj: JObject<'local>,
    idx: jlong,
    value: JString<'local>,
) {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj)?;
        do_tx_op(
            env,
            tx_pointer,
            SetOp {
                obj,
                prop: idx.into(),
                value: value.to_string(),
            },
        )?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setBytesInList<'local>(
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
            SetOp {
                obj,
                prop: idx.into(),
                value: bytes,
            },
        )?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setBoolInList<'local>(
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
            SetOp {
                obj,
                prop: idx.into(),
                value: am::ScalarValue::Boolean(value),
            },
        )?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setDateInList<'local>(
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
            SetOp {
                obj,
                prop: idx.into(),
                value: am::ScalarValue::Timestamp(date_millis),
            },
        )?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setCounterInList<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj: JObject<'local>,
    idx: jlong,
    value: jlong,
) {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj)?;
        do_tx_op(
            env,
            tx_pointer,
            SetOp {
                obj,
                prop: idx.into(),
                value: am::ScalarValue::counter(value),
            },
        )?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setNullInList<'local>(
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
            SetOp {
                obj,
                prop: idx.into(),
                value: am::ScalarValue::Null,
            },
        )
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

struct SetObjOp {
    obj: JavaObjId,
    key: automerge::Prop,
    value: am::ObjType,
}

impl TransactionOp for SetObjOp {
    type Output<'local> = JavaObjId;

    unsafe fn execute<'local, T: Transactable>(
        self,
        env: &jni::Env<'local>,
        tx: &mut T,
    ) -> Result<Self::Output<'local>, jni::errors::Error> {
        Ok(unwrap_or_throw_amg_exc(env, tx.put_object(self.obj, self.key, self.value))?.into())
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setObjectInMap<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj: JObject<'local>,
    key: JString<'local>,
    value: JObject<'local>,
) -> JObject<'local> {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj)?;
        let obj_type = JavaObjType::from_java_enum(env, value)?;
        let key = key.to_string();
        let obj_id = do_tx_op(
            env,
            tx_pointer,
            SetObjOp {
                obj,
                key: key.into(),
                value: obj_type.into(),
            },
        )?;
        obj_id.into_jobject(env)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setObjectInList<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj: JObject<'local>,
    idx: jlong,
    value: JObject<'local>,
) -> JObject<'local> {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj)?;
        let obj_type = JavaObjType::from_java_enum(env, value)?;
        let idx = read_usize(env, idx)?;
        let result = do_tx_op(
            env,
            tx_pointer,
            SetObjOp {
                obj,
                key: idx.into(),
                value: obj_type.into(),
            },
        )?;
        result.into_jobject(env)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}
