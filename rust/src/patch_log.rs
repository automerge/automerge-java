use automerge as am;
use automerge_jni_macros::jni_fn;
use jni::sys::jobject;

use crate::interop::AsPointerObj;

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn createPatchLog(env: jni::JNIEnv, _class: jni::objects::JClass) -> jobject {
    let patch_log = am::PatchLog::new(true, am::patches::TextRepresentation::String);
    patch_log.to_pointer_obj(&env).unwrap()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn freePatchLog(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    patchlog_pointer: jobject,
) {
    let _patch_log = am::PatchLog::owned_from_pointer_obj(&env, patchlog_pointer).unwrap();
}
