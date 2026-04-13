use automerge_jni_macros::jni_fn;
use jni::{
    errors::ThrowRuntimeExAndDefault,
    objects::{JClass, JObject},
};

use super::SomeReadPointer;

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getMarksInDoc<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    obj: JObject<'local>,
    heads: JObject<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::doc(doc).marks(env, obj, heads))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getMarksInTx<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    heads: JObject<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::tx(tx).marks(env, obj, heads))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getMarksAtIndexInDoc<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    obj: JObject<'local>,
    index: jni::sys::jint,
    heads: JObject<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::doc(doc).marks_at_index(env, obj, index, heads))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getMarksAtIndexInTx<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    index: jni::sys::jint,
    heads: JObject<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::tx(tx).marks_at_index(env, obj, index, heads))
        .resolve::<ThrowRuntimeExAndDefault>()
}
