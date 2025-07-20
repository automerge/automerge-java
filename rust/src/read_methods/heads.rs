use automerge_jni_macros::jni_fn;
use jni::sys::jobject;

use super::SomeReadPointer;

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getHeadsInTx(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
) -> jobject {
    SomeReadPointer::tx(tx_pointer).heads(&mut env)
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getHeadsInDoc(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jni::sys::jobject,
) -> jobject {
    SomeReadPointer::doc(doc_pointer).heads(&mut env)
}
