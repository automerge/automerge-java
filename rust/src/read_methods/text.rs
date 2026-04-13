use automerge_jni_macros::jni_fn;
use jni::{
    errors::ThrowRuntimeExAndDefault,
    objects::{JClass, JObject, JObjectArray},
};

use super::SomeReadPointer;

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getTextInTx<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::tx(tx).text(env, obj, None))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getTextInDoc<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    obj: JObject<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::doc(doc).text(env, obj, None))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getTextAtInTx<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    heads: JObjectArray<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::tx(tx).text(env, obj, Some(heads)))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getTextAtInDoc<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    obj: JObject<'local>,
    heads: JObjectArray<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::doc(doc).text(env, obj, Some(heads)))
        .resolve::<ThrowRuntimeExAndDefault>()
}
