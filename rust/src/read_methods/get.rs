use automerge_jni_macros::jni_fn;
use jni::{
    errors::ThrowRuntimeExAndDefault,
    objects::{JClass, JObject, JString},
    sys::jlong,
};

use super::SomeReadPointer;

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getInMapInDoc<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    obj: JObject<'local>,
    key: JString<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::doc(doc).get(env, obj, key))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getInMapInTx<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    key: JString<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::tx(tx).get(env, obj, key))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getInListInDoc<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    obj: JObject<'local>,
    idx: jlong,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::doc(doc).get(env, obj, idx))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getInListInTx<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    idx: jlong,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::tx(tx).get(env, obj, idx))
        .resolve::<ThrowRuntimeExAndDefault>()
}
