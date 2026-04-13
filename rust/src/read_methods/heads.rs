use automerge_jni_macros::jni_fn;
use jni::{
    errors::ThrowRuntimeExAndDefault,
    objects::{JClass, JObject, JObjectArray},
};

use super::SomeReadPointer;

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getHeadsInTx<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
) -> JObjectArray<'local> {
    env.with_env(|env| SomeReadPointer::tx(tx).heads(env))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getHeadsInDoc<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc_pointer: JObject<'local>,
) -> JObjectArray<'local> {
    env.with_env(|env| SomeReadPointer::doc(doc_pointer).heads(env))
        .resolve::<ThrowRuntimeExAndDefault>()
}
