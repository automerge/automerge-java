use automerge::{
    self as am,
    sync::{Message, State as AmSyncState, SyncDoc},
    Automerge,
};
use jni::{
    objects::{JByteArray, JClass, JObjectArray},
    NativeMethod,
};

use crate::interop::{heads_to_jobject_array, unwrap_or_throw_amg_exc, JavaPointer};

use crate::bindings;

const _METHODS: &[NativeMethod] = &[
    ams_native! { static extern fn create_sync_state() -> bindings::SyncStatePointer },
    ams_native! { static extern fn generate_sync_message(state: bindings::SyncStatePointer, doc: bindings::DocPointer) -> bindings::Optional },
    ams_native! { static extern fn receive_sync_message(state: bindings::SyncStatePointer, doc: bindings::DocPointer, message: jbyte[]) },
    ams_native! { static extern fn receive_sync_message_log_patches(state: bindings::SyncStatePointer, doc: bindings::DocPointer, patch_log: bindings::PatchLogPointer, message: jbyte[]) },
    ams_native! { static extern fn encode_sync_state(state: bindings::SyncStatePointer) -> jbyte[] },
    ams_native! { static extern fn decode_sync_state(bytes: jbyte[]) -> bindings::SyncStatePointer },
    ams_native! { static extern fn free_sync_state(state: bindings::SyncStatePointer) },
    ams_native! { static extern fn sync_state_shared_heads(state: bindings::SyncStatePointer) -> bindings::ChangeHash[] },
];

fn create_sync_state<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
) -> jni::errors::Result<bindings::SyncStatePointer<'local>> {
    unsafe { AmSyncState::new().store_as_pointer(env) }
}

fn generate_sync_message<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    state: bindings::SyncStatePointer<'local>,
    doc: bindings::DocPointer<'local>,
) -> jni::errors::Result<bindings::Optional<'local>> {
    let mut state = unsafe { AmSyncState::borrow_from_pointer(env, state)? };
    let doc = unsafe { Automerge::borrow_from_pointer(env, doc)? };
    match doc.generate_sync_message(&mut state) {
        None => bindings::Optional::empty(env),
        Some(m) => {
            let bytes = env.byte_array_from_slice(m.encode().as_slice())?;
            bindings::Optional::of(env, &bytes)
        }
    }
}

fn receive_sync_message<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    state: bindings::SyncStatePointer<'local>,
    doc: bindings::DocPointer<'local>,
    message: JByteArray<'local>,
) -> jni::errors::Result<()> {
    let mut state = unsafe { AmSyncState::borrow_from_pointer(env, state)? };
    let mut doc = unsafe { Automerge::borrow_from_pointer(env, doc)? };
    let message_bytes = env.convert_byte_array(&message)?;
    let msg = unwrap_or_throw_amg_exc(env, Message::decode(&message_bytes))?;
    unwrap_or_throw_amg_exc(env, doc.receive_sync_message(&mut state, msg))
}

fn receive_sync_message_log_patches<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    state: bindings::SyncStatePointer<'local>,
    doc: bindings::DocPointer<'local>,
    patch_log: bindings::PatchLogPointer<'local>,
    message: JByteArray<'local>,
) -> jni::errors::Result<()> {
    let mut state = unsafe { AmSyncState::borrow_from_pointer(env, state)? };
    let mut doc = unsafe { Automerge::borrow_from_pointer(env, doc)? };
    let mut patch_log = unsafe { am::PatchLog::borrow_from_pointer(env, patch_log)? };
    let message_bytes = env.convert_byte_array(&message)?;
    let msg = unwrap_or_throw_amg_exc(env, Message::decode(&message_bytes))?;
    unwrap_or_throw_amg_exc(
        env,
        doc.receive_sync_message_log_patches(&mut state, msg, &mut patch_log),
    )
}

fn encode_sync_state<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    state: bindings::SyncStatePointer<'local>,
) -> jni::errors::Result<JByteArray<'local>> {
    let state = unsafe { AmSyncState::borrow_from_pointer(env, state)? };
    env.byte_array_from_slice(state.encode().as_slice())
}

fn decode_sync_state<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    bytes: JByteArray<'local>,
) -> jni::errors::Result<bindings::SyncStatePointer<'local>> {
    let bytes = env.convert_byte_array(&bytes)?;
    let state = unwrap_or_throw_amg_exc(env, AmSyncState::decode(&bytes))?;
    unsafe { state.store_as_pointer(env) }
}

fn free_sync_state<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    state: bindings::SyncStatePointer<'local>,
) -> jni::errors::Result<()> {
    let _ = unsafe { AmSyncState::take_from_pointer(env, state)? };
    Ok(())
}

fn sync_state_shared_heads<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    state: bindings::SyncStatePointer<'local>,
) -> jni::errors::Result<JObjectArray<'local, bindings::ChangeHash<'local>>> {
    let state = unsafe { AmSyncState::borrow_from_pointer(env, state)? };
    heads_to_jobject_array(env, &state.shared_heads)
}
