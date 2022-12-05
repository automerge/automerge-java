use automerge_jni_macros::jni_fn;
use jni::sys::{jlong, jobject};

use super::SomeReadPointer;

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getListLengthInTx(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
) -> jlong {
    SomeReadPointer::tx(tx_pointer).length(env, obj_pointer, None)
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getListLengthInDoc(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
) -> jlong {
    SomeReadPointer::doc(doc_pointer).length(env, obj_pointer, None)
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getListLengthAtInTx(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    heads_pointer: jobject,
) -> jlong {
    SomeReadPointer::tx(tx_pointer).length(env, obj_pointer, Some(heads_pointer))
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getListLengthAtInDoc(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    heads_pointer: jobject,
) -> jlong {
    SomeReadPointer::doc(doc_pointer).length(env, obj_pointer, Some(heads_pointer))
}
