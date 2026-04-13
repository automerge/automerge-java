use automerge_jni_macros::jni_fn;
use jni::{
    errors::ThrowRuntimeExAndDefault,
    objects::{JClass, JObject, JObjectArray, JString},
    sys::jlong,
};

use super::SomeReadPointer;

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getAtInMapInDoc<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    obj: JObject<'local>,
    key: JString<'local>,
    heads: JObjectArray<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::doc(doc).get_at(env, obj, key, heads))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getAtInMapInTx<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    key: JString<'local>,
    heads: JObjectArray<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::tx(tx).get_at(env, obj, key, heads))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getAtInListInDoc<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    obj: JObject<'local>,
    idx: jlong,
    heads: JObjectArray<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::doc(doc).get_at(env, obj, idx, heads))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getAtInListInTx<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    idx: jlong,
    heads: JObjectArray<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::tx(tx).get_at(env, obj, idx, heads))
        .resolve::<ThrowRuntimeExAndDefault>()
}
