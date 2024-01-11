use automerge_jni_macros::jni_fn;
use jni::sys::jobject;

use super::SomeReadPointer;

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getObjectTypeInDoc(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
) -> jobject {
    SomeReadPointer::doc(doc_pointer).get_object_type(env, obj_pointer)
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getObjectTypeInTx(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
) -> jobject {
    SomeReadPointer::tx(tx_pointer).get_object_type(env, obj_pointer)
}
