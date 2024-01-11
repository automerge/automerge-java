use automerge_jni_macros::jni_fn;
use jni::sys::{jlong, jobject};

use super::SomeReadPointer;

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn makeCursorInDoc(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    index: jlong,
    maybe_heads_pointer: jni::sys::jobject,
) -> jobject {
    SomeReadPointer::doc(doc_pointer).make_cursor(env, obj_pointer, index, maybe_heads_pointer)
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn makeCursorInTx(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    index: jlong,
    maybe_heads_pointer: jni::sys::jobject,
) -> jobject {
    SomeReadPointer::tx(tx_pointer).make_cursor(env, obj_pointer, index, maybe_heads_pointer)
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn lookupCursorIndexInDoc(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    cursor_pointer: jni::sys::jobject,
    maybe_heads_pointer: jni::sys::jobject,
) -> jlong {
    SomeReadPointer::doc(doc_pointer).lookup_cursor_index(
        env,
        obj_pointer,
        cursor_pointer,
        maybe_heads_pointer,
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn lookupCursorIndexInTx(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    cursor_pointer: jni::sys::jobject,
    maybe_heads_pointer: jni::sys::jobject,
) -> jlong {
    SomeReadPointer::tx(tx_pointer).lookup_cursor_index(
        env,
        obj_pointer,
        cursor_pointer,
        maybe_heads_pointer,
    )
}
