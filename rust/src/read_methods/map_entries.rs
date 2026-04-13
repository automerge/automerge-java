use automerge_jni_macros::jni_fn;
use jni::{
    errors::ThrowRuntimeExAndDefault,
    objects::{JClass, JObject, JObjectArray},
};

use super::SomeReadPointer;

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getMapEntriesInTx<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::tx(tx).map_entries(env, obj, None))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getMapEntriesInDoc<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    obj: JObject<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::doc(doc).map_entries(env, obj, None))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getMapEntriesAtInTx<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    heads: JObjectArray<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::tx(tx).map_entries(env, obj, Some(heads)))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getMapEntriesAtInDoc<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    obj: JObject<'local>,
    heads: JObjectArray<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::doc(doc).map_entries(env, obj, Some(heads)))
        .resolve::<ThrowRuntimeExAndDefault>()
}
