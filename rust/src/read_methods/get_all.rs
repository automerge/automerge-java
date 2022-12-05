use automerge_jni_macros::jni_fn;
use jni::{
    objects::JString,
    sys::{jlong, jobject},
};

use super::SomeReadPointer;

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getAllInMapInDoc(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
    obj_pointer: jobject,
    key: JString,
) -> jobject {
    SomeReadPointer::doc(doc_pointer).get_all(env, obj_pointer, key, None)
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getAllInMapInTx(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jobject,
    obj_pointer: jobject,
    key: JString,
) -> jobject {
    SomeReadPointer::tx(tx_pointer).get_all(env, obj_pointer, key, None)
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getAllInListInDoc(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
    obj_pointer: jobject,
    idx: jlong,
) -> jobject {
    SomeReadPointer::doc(doc_pointer).get_all(env, obj_pointer, idx, None)
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getAllInListInTx(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jobject,
    obj_pointer: jobject,
    idx: jlong,
) -> jobject {
    SomeReadPointer::tx(tx_pointer).get_all(env, obj_pointer, idx, None)
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getAllAtInMapInDoc(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
    obj_pointer: jobject,
    key: JString,
    heads_pointer: jobject,
) -> jobject {
    SomeReadPointer::doc(doc_pointer).get_all(env, obj_pointer, key, Some(heads_pointer))
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getAllAtInMapInTx(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jobject,
    obj_pointer: jobject,
    key: JString,
    heads_pointer: jobject,
) -> jobject {
    SomeReadPointer::tx(tx_pointer).get_all(env, obj_pointer, key, Some(heads_pointer))
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getAllAtInListInDoc(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
    obj_pointer: jobject,
    idx: jlong,
    heads_pointer: jobject,
) -> jobject {
    SomeReadPointer::doc(doc_pointer).get_all(env, obj_pointer, idx, Some(heads_pointer))
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getAllAtInListInTx(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jobject,
    obj_pointer: jobject,
    idx: jlong,
    heads_pointer: jobject,
) -> jobject {
    SomeReadPointer::tx(tx_pointer).get_all(env, obj_pointer, idx, Some(heads_pointer))
}
