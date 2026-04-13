use automerge::{self as am};
use automerge_jni_macros::jni_fn;
use jni::{
    errors::ThrowRuntimeExAndDefault,
    objects::{JClass, JObject},
};

use crate::interop::JavaPointer;

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn createPatchLog<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
) -> JObject<'local> {
    env.with_env(|env| {
        let patch_log = am::PatchLog::new(true);
        patch_log.store_as_pointer(env)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn freePatchLog<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    patchlog: JObject<'local>,
) {
    env.with_env(|env| {
        let _patch_log = am::PatchLog::take_from_pointer(env, patchlog)?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}
