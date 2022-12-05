use automerge_jni_macros::jni_fn;
use jni::sys::jobject;

use super::SomeReadPointer;

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getTextInTx(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
) -> jobject {
    SomeReadPointer::tx(tx_pointer).text(env, obj_pointer, None)
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getTextInDoc(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
) -> jobject {
    SomeReadPointer::doc(doc_pointer).text(env, obj_pointer, None)
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getTextAtInTx(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    heads_pointer: jobject,
) -> jobject {
    SomeReadPointer::tx(tx_pointer).text(env, obj_pointer, Some(heads_pointer))
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getTextAtInDoc(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    heads_pointer: jobject,
) -> jobject {
    SomeReadPointer::doc(doc_pointer).text(env, obj_pointer, Some(heads_pointer))
}
