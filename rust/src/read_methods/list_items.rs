use automerge_jni_macros::jni_fn;
use jni::sys::jobject;

use super::SomeReadPointer;

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getListItemsInTx(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
) -> jobject {
    SomeReadPointer::tx(tx_pointer).list_items(env, obj_pointer, None)
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getListItemsInDoc(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
) -> jobject {
    SomeReadPointer::doc(doc_pointer).list_items(env, obj_pointer, None)
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getListItemsAtInTx(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    heads_pointer: jobject,
) -> jobject {
    SomeReadPointer::tx(tx_pointer).list_items(env, obj_pointer, Some(heads_pointer))
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getListItemsAtInDoc(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    heads_pointer: jobject,
) -> jobject {
    SomeReadPointer::doc(doc_pointer).list_items(env, obj_pointer, Some(heads_pointer))
}
