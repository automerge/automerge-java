use automerge_jni_macros::jni_fn;

use super::SomeReadPointer;

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getMarksInDoc(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    heads_pointer: jni::sys::jobject,
) -> jni::sys::jobject {
    SomeReadPointer::doc(doc_pointer).marks(env, obj_pointer, heads_pointer)
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getMarksInTx(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    heads_pointer: jni::sys::jobject,
) -> jni::sys::jobject {
    SomeReadPointer::tx(tx_pointer).marks(env, obj_pointer, heads_pointer)
}
