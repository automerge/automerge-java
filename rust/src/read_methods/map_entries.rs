use automerge_jni_macros::jni_fn;
use jni::sys::jobject;

use super::SomeReadPointer;

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getMapEntriesInTx(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jobject,
    obj_pointer: jobject,
) -> jobject {
    SomeReadPointer::tx(tx_pointer).map_entries(env, obj_pointer, None)
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getMapEntriesInDoc(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
    obj_pointer: jobject,
) -> jobject {
    SomeReadPointer::doc(doc_pointer).map_entries(env, obj_pointer, None)
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getMapEntriesAtInTx(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jobject,
    obj_pointer: jobject,
    heads_pointer: jobject,
) -> jobject {
    SomeReadPointer::tx(tx_pointer).map_entries(env, obj_pointer, Some(heads_pointer))
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getMapEntriesAtInDoc(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
    obj_pointer: jobject,
    heads_pointer: jobject,
) -> jobject {
    SomeReadPointer::doc(doc_pointer).map_entries(env, obj_pointer, Some(heads_pointer))
}
