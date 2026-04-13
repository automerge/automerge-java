use automerge::{
    self as am,
    sync::{Message, State as SyncState, SyncDoc},
    Automerge,
};
use automerge_jni_macros::jni_fn;
use jni::{
    errors::ThrowRuntimeExAndDefault,
    objects::{JByteArray, JClass, JObject, JObjectArray},
};

use crate::{
    interop::{changehash_to_jobject, unwrap_or_throw_amg_exc, JavaPointer, CHANGEHASH_CLASS},
    java_option::{make_empty_option, make_optional},
};

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn createSyncState<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
) -> JObject<'local> {
    env.with_env(|env| {
        let state = SyncState::new();
        state.store_as_pointer(env)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn generateSyncMessage<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    state: JObject<'local>,
    doc: JObject<'local>,
) -> JObject<'local> {
    env.with_env(|env| {
        let mut state = SyncState::borrow_from_pointer(env, state)?;
        let doc = Automerge::borrow_from_pointer(env, doc)?;
        match doc.generate_sync_message(&mut state) {
            None => make_empty_option(env),
            Some(m) => {
                let bytes = env.byte_array_from_slice(m.encode().as_slice())?;
                make_optional(env, (&bytes).into())
            }
        }
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn receiveSyncMessage<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    state: JObject<'local>,
    doc: JObject<'local>,
    message: JByteArray<'local>,
) {
    env.with_env(|env| {
        let mut state = SyncState::borrow_from_pointer(env, state)?;
        let mut doc = Automerge::borrow_from_pointer(env, doc)?;
        let message_bytes = env.convert_byte_array(&message)?;
        let message = unwrap_or_throw_amg_exc(env, Message::decode(&message_bytes))?;
        unwrap_or_throw_amg_exc(env, doc.receive_sync_message(&mut state, message))
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn receiveSyncMessageLogPatches<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    state: JObject<'local>,
    doc: JObject<'local>,
    patch_log: JObject<'local>,
    message: JByteArray<'local>,
) {
    env.with_env(|env| {
        let mut state = SyncState::borrow_from_pointer(env, state)?;
        let mut doc = Automerge::borrow_from_pointer(env, doc)?;
        let mut patch_log = am::PatchLog::borrow_from_pointer(env, patch_log)?;
        let message_bytes = env.convert_byte_array(&message)?;
        let message = unwrap_or_throw_amg_exc(env, Message::decode(&message_bytes))?;
        unwrap_or_throw_amg_exc(
            env,
            doc.receive_sync_message_log_patches(&mut state, message, &mut patch_log),
        )
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn encodeSyncState<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    state: JObject<'local>,
) -> JByteArray<'local> {
    env.with_env(|env| {
        let state = SyncState::borrow_from_pointer(env, state)?;
        env.byte_array_from_slice(state.encode().as_slice())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn decodeSyncState<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    bytes: JByteArray<'local>,
) -> JObject<'local> {
    env.with_env(|env| {
        let bytes = env.convert_byte_array(&bytes)?;
        let state = unwrap_or_throw_amg_exc(env, SyncState::decode(&bytes))?;
        state.store_as_pointer(env)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn freeSyncState<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    state: JObject<'local>,
) {
    env.with_env(|env| {
        let _state = SyncState::take_from_pointer(env, state)?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn syncStateSharedHeads<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    state: JObject<'local>,
) -> JObjectArray<'local> {
    env.with_env(|env| {
        let state = SyncState::borrow_from_pointer(env, state)?;

        let heads_arr = env.new_object_array(
            state.shared_heads.len() as i32,
            CHANGEHASH_CLASS,
            JObject::null(),
        )?;
        for (i, head) in state.shared_heads.iter().enumerate() {
            let hash = changehash_to_jobject(env, head)?;
            heads_arr.set_element(env, i, hash)?;
        }
        Ok::<_, jni::errors::Error>(heads_arr)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}
