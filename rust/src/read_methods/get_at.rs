use automerge_jni_macros::jni_fn;
use jni::{
    objects::JString,
    sys::{jlong, jobject},
};

use super::SomeReadPointer;

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getAtInMapInDoc(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    key: JString,
    heads: jobject,
) -> jni::sys::jobject {
    SomeReadPointer::doc(doc_pointer).get_at(env, obj_pointer, key, heads)
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getAtInMapInTx(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    key: JString,
    heads: jobject,
) -> jni::sys::jobject {
    SomeReadPointer::tx(tx_pointer).get_at(env, obj_pointer, key, heads)
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getAtInListInDoc(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    idx: jlong,
    heads: jobject,
) -> jni::sys::jobject {
    SomeReadPointer::doc(doc_pointer).get_at(env, obj_pointer, idx, heads)
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getAtInListInTx(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    idx: jlong,
    heads: jobject,
) -> jni::sys::jobject {
    SomeReadPointer::tx(tx_pointer).get_at(env, obj_pointer, idx, heads)
}
