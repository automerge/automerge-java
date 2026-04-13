use automerge_jni_macros::jni_fn;
use jni::{
    errors::ThrowRuntimeExAndDefault,
    objects::{JClass, JObject},
    sys::jlong,
};

use super::SomeReadPointer;

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn makeCursorInDoc<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    obj: JObject<'local>,
    index: jlong,
    heads: JObject<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::doc(doc).make_cursor(env, obj, index, heads))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn makeCursorInTx<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    index: jlong,
    heads: JObject<'local>,
) -> JObject<'local> {
    env.with_env(|env| SomeReadPointer::tx(tx).make_cursor(env, obj, index, heads))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn lookupCursorIndexInDoc<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    obj: JObject<'local>,
    cursor: JObject<'local>,
    heads: JObject<'local>,
) -> jlong {
    env.with_env(|env| SomeReadPointer::doc(doc).lookup_cursor_index(env, obj, cursor, heads))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn lookupCursorIndexInTx<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    cursor: JObject<'local>,
    heads: JObject<'local>,
) -> jlong {
    env.with_env(|env| SomeReadPointer::tx(tx).lookup_cursor_index(env, obj, cursor, heads))
        .resolve::<ThrowRuntimeExAndDefault>()
}
