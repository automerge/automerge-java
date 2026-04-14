use automerge::{self as am};
use jni::{objects::JClass, NativeMethod};

use crate::{bindings, interop::JavaPointer};

const _METHODS: &[NativeMethod] = &[
    ams_native! { static extern fn create_patch_log() -> bindings::PatchLogPointer },
    ams_native! { static extern fn free_patch_log(patchlog: bindings::PatchLogPointer) },
];

fn create_patch_log<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
) -> jni::errors::Result<bindings::PatchLogPointer<'local>> {
    unsafe { am::PatchLog::new(true).store_as_pointer(env) }
}

fn free_patch_log<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    patchlog: bindings::PatchLogPointer<'local>,
) -> jni::errors::Result<()> {
    let _patch_log = unsafe { am::PatchLog::take_from_pointer(env, patchlog)? };
    Ok(())
}
