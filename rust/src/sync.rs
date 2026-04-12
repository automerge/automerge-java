use automerge::{
    self as am,
    sync::{Message, State as SyncState, SyncDoc},
    Automerge,
};
use automerge_jni_macros::jni_fn;
use jni::{
    objects::{JObject, JPrimitiveArray},
    sys::{jbyteArray, jobject},
};

use crate::{
    interop::{changehash_to_jobject, throw_amg_exc_or_fatal, JavaPointer, CHANGEHASH_CLASS},
    java_option::{make_empty_option, make_optional},
};

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn createSyncState(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
) -> jobject {
    let state = SyncState::new();
    state.store_as_pointer(&mut env).unwrap().into_raw()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn generateSyncMessage(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    state_pointer: jobject,
    doc_pointer: jobject,
) -> jobject {
    let mut env_for_state = env.unsafe_clone();
    let mut state = SyncState::borrow_from_pointer(&mut env_for_state, state_pointer).unwrap();
    let mut env_for_doc = env.unsafe_clone();
    let doc = Automerge::borrow_from_pointer(&mut env_for_doc, doc_pointer).unwrap();
    match doc.generate_sync_message(&mut *state) {
        None => make_empty_option(&mut env).unwrap().into_raw(),
        Some(m) => {
            let bytes = env.byte_array_from_slice(m.encode().as_slice()).unwrap();
            make_optional(&mut env, (&bytes).into()).unwrap().into_raw()
        }
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn receiveSyncMessage(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    state_pointer: jobject,
    doc_pointer: jobject,
    message_pointer: jbyteArray,
) {
    let mut env_for_state = env.unsafe_clone();
    let mut state = SyncState::borrow_from_pointer(&mut env_for_state, state_pointer).unwrap();
    let mut env_for_doc = env.unsafe_clone();
    let mut doc = Automerge::borrow_from_pointer(&mut env_for_doc, doc_pointer).unwrap();
    let message_pointer = JPrimitiveArray::from_raw(message_pointer);
    let message_bytes = env.convert_byte_array(&message_pointer).unwrap();
    let message = match Message::decode(&message_bytes) {
        Ok(m) => m,
        Err(e) => {
            throw_amg_exc_or_fatal(&mut env, e.to_string());
            return;
        }
    };
    match doc.receive_sync_message(&mut *state, message) {
        Ok(()) => {}
        Err(e) => {
            throw_amg_exc_or_fatal(&mut env, e.to_string());
        }
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn receiveSyncMessageLogPatches(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    state_pointer: jobject,
    doc_pointer: jobject,
    patch_log_pointer: jobject,
    message_pointer: jobject,
) {
    let mut env_for_state = env.unsafe_clone();
    let mut state = SyncState::borrow_from_pointer(&mut env_for_state, state_pointer).unwrap();
    let mut env_for_doc = env.unsafe_clone();
    let mut doc = Automerge::borrow_from_pointer(&mut env_for_doc, doc_pointer).unwrap();
    let mut env_for_patch_log = env.unsafe_clone();
    let mut patch_log =
        am::PatchLog::borrow_from_pointer(&mut env_for_patch_log, patch_log_pointer).unwrap();
    let message_pointer = JPrimitiveArray::from_raw(message_pointer);
    let message_bytes = env.convert_byte_array(&message_pointer).unwrap();
    let message = match Message::decode(&message_bytes) {
        Ok(m) => m,
        Err(e) => {
            throw_amg_exc_or_fatal(&mut env, e.to_string());
            return;
        }
    };
    match doc.receive_sync_message_log_patches(&mut *state, message, &mut *patch_log) {
        Ok(_) => {}
        Err(e) => {
            throw_amg_exc_or_fatal(&mut env, e.to_string());
        }
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn encodeSyncState(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    state_pointer: jobject,
) -> jobject {
    let mut env_for_state = env.unsafe_clone();
    let state = SyncState::borrow_from_pointer(&mut env_for_state, state_pointer).unwrap();
    let bytes = env
        .byte_array_from_slice(state.encode().as_slice())
        .unwrap();
    bytes.into_raw()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn decodeSyncState(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    bytes_pointer: jbyteArray,
) -> jobject {
    let bytes_pointer = JPrimitiveArray::from_raw(bytes_pointer);
    let bytes = env.convert_byte_array(&bytes_pointer).unwrap();
    match SyncState::decode(&bytes) {
        Ok(state) => state.store_as_pointer(&mut env).unwrap().into_raw(),
        Err(e) => {
            throw_amg_exc_or_fatal(&mut env, e.to_string());
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn freeSyncState(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    state_pointer: jobject,
) {
    let _state = SyncState::take_from_pointer(&mut env, state_pointer).unwrap();
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn syncStateSharedHeads(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    state_pointer: jobject,
) -> jobject {
    let mut env_for_state = env.unsafe_clone();
    let state = SyncState::borrow_from_pointer(&mut env_for_state, state_pointer).unwrap();

    let heads_arr = env
        .new_object_array(
            state.shared_heads.len() as i32,
            CHANGEHASH_CLASS,
            JObject::null(),
        )
        .unwrap();
    for (i, head) in state.shared_heads.iter().enumerate() {
        let hash = changehash_to_jobject(&mut env, head).unwrap();
        env.set_object_array_element(&heads_arr, i as i32, hash)
            .unwrap();
    }
    heads_arr.into_raw()
}
