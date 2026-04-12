use automerge::{self as am};
use automerge_jni_macros::jni_fn;
use jni::sys::jobject;

use crate::interop::JavaPointer;

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn createPatchLog(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
) -> jobject {
    let patch_log = am::PatchLog::new(true);
    patch_log.store_as_pointer(&mut env).unwrap().into_raw()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn freePatchLog(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    patchlog_pointer: jobject,
) {
    let _patch_log = am::PatchLog::take_from_pointer(&mut env, patchlog_pointer).unwrap();
}
