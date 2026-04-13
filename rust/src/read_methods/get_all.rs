use automerge_jni_macros::jni_fn;
use jni::{
    errors::ThrowRuntimeExAndDefault,
    objects::{JClass, JObject, JObjectArray, JString},
    sys::jlong,
};

use super::SomeReadPointer;

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getAllInMapInDoc<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    obj: JObject<'local>,
    key: JString<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::doc(doc).get_all(env, obj, key, None))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getAllInMapInTx<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    key: JString<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::tx(tx).get_all(env, obj, key, None))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getAllInListInDoc<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    obj: JObject<'local>,
    idx: jlong,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::doc(doc).get_all(env, obj, idx, None))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getAllInListInTx<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    idx: jlong,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::tx(tx).get_all(env, obj, idx, None))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getAllAtInMapInDoc<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    obj: JObject<'local>,
    key: JString<'local>,
    heads: JObjectArray<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::doc(doc).get_all(env, obj, key, Some(heads)))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getAllAtInMapInTx<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    key: JString<'local>,
    heads: JObjectArray<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::tx(tx).get_all(env, obj, key, Some(heads)))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getAllAtInListInDoc<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    obj: JObject<'local>,
    idx: jlong,
    heads: JObjectArray<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::doc(doc).get_all(env, obj, idx, Some(heads)))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getAllAtInListInTx<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    idx: jlong,
    heads: JObjectArray<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::tx(tx).get_all(env, obj, idx, Some(heads)))
        .resolve::<ThrowRuntimeExAndDefault>()
}
