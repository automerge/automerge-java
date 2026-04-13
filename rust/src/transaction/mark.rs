use am::transaction::Transactable;
use automerge::{self as am, marks::ExpandMark, ScalarValue};
use automerge_jni_macros::jni_fn;
use jni::{
    errors::ThrowRuntimeExAndDefault,
    jni_sig, jni_str,
    objects::{JByteArray, JClass, JObject, JString},
    sys::{jboolean, jdouble, jlong},
};

use crate::{
    expand_mark,
    interop::{read_u64, unwrap_or_throw_amg_exc},
    obj_id::JavaObjId,
};

use super::{do_tx_op, TransactionOp};

struct MarkOp {
    obj: JavaObjId,
    start: usize,
    end: usize,
    name: String,
    value: am::ScalarValue,
    expand: ExpandMark,
}

impl TransactionOp for MarkOp {
    type Output<'local> = ();

    unsafe fn execute<'local, T: Transactable>(
        self,
        env: &jni::Env<'local>,
        tx: &mut T,
    ) -> Result<Self::Output<'local>, jni::errors::Error> {
        let mark = am::marks::Mark::new(self.name, self.value, self.start, self.end);
        unwrap_or_throw_amg_exc(env, tx.mark(self.obj, mark, self.expand))
    }
}

#[allow(clippy::too_many_arguments)]
unsafe fn do_mark<'local, V: Into<ScalarValue>>(
    env: &mut jni::Env<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    name: JString<'local>,
    start: jlong,
    end: jlong,
    value: V,
    expand: JObject<'local>,
) -> Result<(), jni::errors::Error> {
    let obj = JavaObjId::from_jobject(env, obj)?;
    let expand = expand_mark::from_java(env, expand)?;
    do_tx_op(
        env,
        tx,
        MarkOp {
            obj,
            start: start as usize,
            end: end as usize,
            name: name.to_string(),
            value: value.into(),
            expand,
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn markUint<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx_pointer: JObject<'local>,
    obj: JObject<'local>,
    name: JString<'local>,
    start: jlong,
    end: jlong,
    value: jlong,
    expand_obj: JObject<'local>,
) {
    env.with_env(|env| {
        let value = read_u64(env, value)?;
        do_mark(env, tx_pointer, obj, name, start, end, value, expand_obj)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn markInt<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    name: JString<'local>,
    start: jlong,
    end: jlong,
    value: jlong,
    expand: JObject<'local>,
) {
    env.with_env(|env| do_mark(env, tx, obj, name, start, end, value, expand))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn markDouble<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    name: JString<'local>,
    start: jlong,
    end: jlong,
    value: jdouble,
    expand: JObject<'local>,
) {
    env.with_env(|env| do_mark(env, tx, obj, name, start, end, value, expand))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn markBytes<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    name: JString<'local>,
    start: jlong,
    end: jlong,
    value: JByteArray<'local>,
    expand: JObject<'local>,
) {
    env.with_env(|env| {
        let value = env.convert_byte_array(value)?;
        do_mark(env, tx, obj, name, start, end, value, expand)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn markString<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    name: JString<'local>,
    start: jlong,
    end: jlong,
    value: JString<'local>,
    expand: JObject<'local>,
) {
    env.with_env(|env| {
        let value = value.to_string();
        do_mark(env, tx, obj, name, start, end, value, expand)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn markCounter<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    name: JString<'local>,
    start: jlong,
    end: jlong,
    value: jlong,
    expand: JObject<'local>,
) {
    env.with_env(|env| {
        let value = ScalarValue::Counter(value.into());
        do_mark(env, tx, obj, name, start, end, value, expand)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn markDate<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    name: JString<'local>,
    start: jlong,
    end: jlong,
    date: JObject<'local>,
    expand: JObject<'local>,
) {
    env.with_env(|env| {
        let date_millis = env
            .call_method(date, jni_str!("getTime"), jni_sig!("()J"), &[])?
            .j()?;
        do_mark(
            env,
            tx,
            obj,
            name,
            start,
            end,
            ScalarValue::Timestamp(date_millis),
            expand,
        )
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn markBool<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    name: JString<'local>,
    start: jlong,
    end: jlong,
    value: jboolean,
    expand: JObject<'local>,
) {
    env.with_env(|env| {
        do_mark(
            env,
            tx,
            obj,
            name,
            start,
            end,
            am::ScalarValue::Boolean(value),
            expand,
        )
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn markNull<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    name: JString<'local>,
    start: jlong,
    end: jlong,
    expand: JObject<'local>,
) {
    env.with_env(|env| {
        do_mark(
            env,
            tx,
            obj,
            name,
            start,
            end,
            am::ScalarValue::Null,
            expand,
        )
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

struct Unmark {
    obj: JavaObjId,
    start: usize,
    end: usize,
    name: String,
    expand: ExpandMark,
}

impl TransactionOp for Unmark {
    type Output<'local> = ();

    unsafe fn execute<'local, T: Transactable>(
        self,
        env: &jni::Env<'local>,
        tx: &mut T,
    ) -> Result<Self::Output<'local>, jni::errors::Error> {
        unwrap_or_throw_amg_exc(
            env,
            tx.unmark(self.obj, &self.name, self.start, self.end, self.expand),
        )
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn unMark<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    name: JString<'local>,
    start: jlong,
    end: jlong,
    expand: JObject<'local>,
) {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj)?;
        let expand = expand_mark::from_java(env, expand)?;
        let name = name.to_string();
        do_tx_op(
            env,
            tx,
            Unmark {
                obj,
                start: start as usize,
                end: end as usize,
                name,
                expand,
            },
        )
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}
