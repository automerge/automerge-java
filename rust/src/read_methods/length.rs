use automerge_jni_macros::jni_fn;
use jni::{
    errors::ThrowRuntimeExAndDefault,
    objects::{JClass, JObject, JObjectArray},
    sys::jlong,
};

use super::SomeReadPointer;

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getListLengthInTx<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
) -> jlong {
    env.with_env(|env| SomeReadPointer::tx(tx).length(env, obj, None))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getListLengthInDoc<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    obj: JObject<'local>,
) -> jlong {
    env.with_env(|env| SomeReadPointer::doc(doc).length(env, obj, None))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getListLengthAtInTx<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    heads: JObjectArray<'local>,
) -> jlong {
    env.with_env(|env| SomeReadPointer::tx(tx).length(env, obj, Some(heads)))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getListLengthAtInDoc<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    obj: JObject<'local>,
    heads: JObjectArray<'local>,
) -> jlong {
    env.with_env(|env| SomeReadPointer::doc(doc).length(env, obj, Some(heads)))
        .resolve::<ThrowRuntimeExAndDefault>()
}
