use automerge_jni_macros::jni_fn;
use jni::{objects::JString, sys::jlong};

use super::SomeReadPointer;

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getInMapInDoc(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    key: JString,
) -> jni::sys::jobject {
    SomeReadPointer::doc(doc_pointer).get(env, obj_pointer, key)
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getInMapInTx(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    key: JString,
) -> jni::sys::jobject {
    SomeReadPointer::tx(tx_pointer).get(env, obj_pointer, key)
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getInListInDoc(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    idx: jlong,
) -> jni::sys::jobject {
    SomeReadPointer::doc(doc_pointer).get(env, obj_pointer, idx)
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getInListInTx(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    idx: jlong,
) -> jni::sys::jobject {
    SomeReadPointer::tx(tx_pointer).get(env, obj_pointer, idx)
}
