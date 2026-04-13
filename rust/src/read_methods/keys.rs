use automerge_jni_macros::jni_fn;
use jni::{
    errors::ThrowRuntimeExAndDefault,
    objects::{JClass, JObject, JObjectArray},
};

use super::SomeReadPointer;

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getKeysInDoc<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    obj: JObject<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::doc(doc).keys(env, obj, None))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getKeysInTx<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::tx(tx).keys(env, obj, None))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getKeysAtInDoc<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    obj: JObject<'local>,
    heads: JObjectArray<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::doc(doc).keys(env, obj, Some(heads)))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getKeysAtInTx<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    heads: JObjectArray<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::tx(tx).keys(env, obj, Some(heads)))
        .resolve::<ThrowRuntimeExAndDefault>()
}
