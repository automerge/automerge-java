//! Native methods on `RepoSys` that drive samod `DocumentActor` instances,
//! plus the `DocActorResult` conversion and the IO task / message
//! conversion helpers those actors depend on.

use jni::{
    jni_str,
    objects::{JByteArray, JClass, JList, JObject},
    strings::JNIString,
    sys::{jboolean, jlong},
    NativeMethod,
};
use samod_core::actors::document::io::{DocumentIoResult, DocumentIoTask};
use samod_core::actors::document::{DocActorResult, DocumentActor};
use samod_core::actors::hub::io::{HubIoAction, HubIoResult};
use samod_core::actors::{DocToHubMsg, HubToDocMsg};
use samod_core::io::{IoResult, IoTask, IoTaskId};
use samod_core::DocumentChanged;
use samod_core::UnixTimestamp;

use crate::interop::JavaPointer;
use crate::repo::bindings as repo_bindings;
use crate::repo::storage::{storage_result_from_java, storage_task_to_java};
use crate::{
    bindings::{self, ArrayList, HashMap as JHashMap},
    interop::throw_illegal_argument,
};

const _METHODS: &[NativeMethod] = &[
    repo_native! { static extern fn document_actor_handle_msg(
        actor: repo_bindings::DocumentActorPointer,
        timestamp: jlong,
        msg: repo_bindings::HubToDocMsg
    ) -> repo_bindings::DocActorResult },
    repo_native! { static extern fn document_actor_handle_io_complete(
        actor: repo_bindings::DocumentActorPointer,
        timestamp: jlong,
        io_result: repo_bindings::IoResult
    ) -> repo_bindings::DocActorResult },
    repo_native! { static extern fn document_actor_with_document(
        actor: repo_bindings::DocumentActorPointer,
        timestamp: jlong,
        fn_obj: crate::bindings::Function
    ) -> repo_bindings::WithDocResult },
    repo_native! { static extern fn document_actor_is_stopped(actor: repo_bindings::DocumentActorPointer) -> jboolean },
    repo_native! { static extern fn free_document_actor(actor: repo_bindings::DocumentActorPointer) },
    repo_native! { static extern fn free_hub_to_doc_msg(msg: repo_bindings::HubToDocMsg) },
    repo_native! { static extern fn free_doc_to_hub_msg(msg: repo_bindings::DocToHubMsg) },
];

// --- Lifecycle --------------------------------------------------------

fn document_actor_handle_msg<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    actor: repo_bindings::DocumentActorPointer<'local>,
    timestamp: jlong,
    msg: repo_bindings::HubToDocMsg<'local>,
) -> jni::errors::Result<repo_bindings::DocActorResult<'local>> {
    let timestamp = UnixTimestamp::from_millis(timestamp as u128);
    let msg = unsafe { HubToDocMsg::take_from_pointer(env, msg)? };
    let result = {
        let mut actor_guard = unsafe { DocumentActor::borrow_from_pointer(env, actor)? };
        match actor_guard.handle_message(timestamp, msg) {
            Ok(r) => r,
            Err(e) => {
                env.throw_new(
                    jni::jni_str!("java/lang/RuntimeException"),
                    JNIString::from(format!("DocumentActor::handle_message failed: {:?}", e)),
                )?;
                return Err(jni::errors::Error::JavaException);
            }
        }
    };
    doc_actor_result_to_java(env, result)
}

fn document_actor_handle_io_complete<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    actor: repo_bindings::DocumentActorPointer<'local>,
    timestamp: jlong,
    io_result: repo_bindings::IoResult<'local>,
) -> jni::errors::Result<repo_bindings::DocActorResult<'local>> {
    let timestamp = UnixTimestamp::from_millis(timestamp as u128);
    let io_result = document_io_result_from_java(env, io_result)?;
    let result = {
        let mut actor_guard = unsafe { DocumentActor::borrow_from_pointer(env, actor)? };
        match actor_guard.handle_io_complete(timestamp, io_result) {
            Ok(r) => r,
            Err(e) => {
                env.throw_new(
                    jni::jni_str!("java/lang/RuntimeException"),
                    JNIString::from(format!("DocumentActor::handle_io_complete failed: {:?}", e)),
                )?;
                return Err(jni::errors::Error::JavaException);
            }
        }
    };
    doc_actor_result_to_java(env, result)
}

fn document_actor_with_document<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    actor: repo_bindings::DocumentActorPointer<'local>,
    timestamp: jlong,
    fn_obj: bindings::Function<'local>,
) -> jni::errors::Result<repo_bindings::WithDocResult<'local>> {
    let timestamp = UnixTimestamp::from_millis(timestamp as u128);

    // Run the samod-core actor's `with_document`. Inside the closure we
    // temporarily swap the actor's Automerge out so we can hand it to Java
    // wrapped as an `org.automerge.Document` pointer, then we swap it back
    // once the user's `Function` returns.
    let samod_result = {
        let mut actor_guard = unsafe { DocumentActor::borrow_from_pointer(env, actor)? };
        let outcome =
            actor_guard.with_document(timestamp, |am| run_user_callback(env, &fn_obj, am));
        match outcome {
            Ok(r) => r,
            Err(e) => {
                // Only synthesise an exception if the user callback didn't
                // already leave one pending — `throw_new` would clobber it.
                if !env.exception_check() {
                    env.throw_new(
                        jni_str!("java/lang/RuntimeException"),
                        JNIString::from(format!("DocumentActor::with_document failed: {:?}", e)),
                    )?;
                }
                return Err(jni::errors::Error::JavaException);
            }
        }
    };

    // samod_result.value carries the JNI outcome of the user callback; an
    // Err here means either a real JNI error or that the user's callback
    // threw a Java exception, which is already pending and must propagate.
    let callback_value = samod_result.value?;
    let actor_result_java = doc_actor_result_to_java(env, samod_result.actor_result)?;
    repo_bindings::WithDocResult::new(env, &callback_value, &actor_result_java)
}

/// Called from inside `DocumentActor::with_document` with a mutable
/// reference to the actor's `Automerge`. Moves the doc into a Java-owned
/// `DocPointer`, wraps it in an `org.automerge.Document`, runs the user's
/// `Function.apply`, evicts the Document's pointer reference, then moves
/// the `Automerge` back into place.
fn run_user_callback<'local>(
    env: &mut jni::Env<'local>,
    fn_obj: &bindings::Function<'local>,
    am: &mut automerge::Automerge,
) -> jni::errors::Result<JObject<'local>> {
    // Swap in a fresh placeholder so we can hand the real Automerge to
    // Java. On every exit path below we restore the real one before
    // returning to samod-core, which needs to observe the post-callback
    // state of `am` to compute its actor_result.
    let placeholder = automerge::Automerge::new();
    let taken = std::mem::replace(am, placeholder);
    let doc_ptr = unsafe { taken.store_as_pointer(env)? };

    let callback_outcome = invoke_apply(env, fn_obj, &doc_ptr);

    // Always try to reclaim the Automerge behind `doc_ptr`. If it fails
    // we leave the placeholder in place — samod will proceed against an
    // empty doc, which is recoverable only by the caller.
    match unsafe { automerge::Automerge::take_from_pointer(env, &doc_ptr) } {
        Ok(doc_back) => *am = doc_back,
        Err(take_err) => {
            if callback_outcome.is_ok() {
                return Err(take_err);
            }
        }
    }

    callback_outcome
}

fn invoke_apply<'local>(
    env: &mut jni::Env<'local>,
    fn_obj: &bindings::Function<'local>,
    doc_ptr: &bindings::DocPointer<'local>,
) -> jni::errors::Result<JObject<'local>> {
    let document_obj = bindings::Document::new(env, doc_ptr)?;

    let apply_result = fn_obj.apply(env, &document_obj);

    // Whatever happened, evict the Document's Java-side pointer reference
    // so any stashed handle stops working. Swallow failures here — if
    // `invalidate` itself fails (e.g. because an exception is already
    // pending from the callback) we prefer to surface the original error.
    let _ = document_obj.invalidate(env);

    apply_result
}

fn document_actor_is_stopped<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    actor: repo_bindings::DocumentActorPointer<'local>,
) -> jni::errors::Result<jboolean> {
    let stopped = {
        let actor_guard = unsafe { DocumentActor::borrow_from_pointer(env, actor)? };
        actor_guard.is_stopped()
    };
    Ok(stopped as jboolean)
}

fn free_document_actor<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    actor: repo_bindings::DocumentActorPointer<'local>,
) -> jni::errors::Result<()> {
    let _actor = unsafe { DocumentActor::take_from_pointer(env, actor)? };
    Ok(())
}

fn free_hub_to_doc_msg<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    msg: repo_bindings::HubToDocMsg<'local>,
) -> jni::errors::Result<()> {
    let _msg = unsafe { HubToDocMsg::take_from_pointer(env, msg)? };
    Ok(())
}

fn free_doc_to_hub_msg<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    msg: repo_bindings::DocToHubMsg<'local>,
) -> jni::errors::Result<()> {
    let _msg = unsafe { DocToHubMsg::take_from_pointer(env, msg)? };
    Ok(())
}

// --- DocActorResult (Rust → Java) ------------------------------------

pub(crate) fn doc_actor_result_to_java<'local>(
    env: &mut jni::Env<'local>,
    result: DocActorResult,
) -> jni::errors::Result<repo_bindings::DocActorResult<'local>> {
    let DocActorResult {
        io_tasks,
        outgoing_messages,
        ephemeral_messages,
        change_events,
        stopped,
        peer_state_changes: _,
        // Fields introduced in samod-core 0.9 that we don't surface yet:
        sync_message_stats: _,
        pending_sync_messages: _,
        ..
    } = result;

    let io_tasks_java = doc_io_task_list_to_java(env, io_tasks)?;
    let outgoing_java = doc_to_hub_msg_list_to_java(env, outgoing_messages)?;
    let ephemeral_java = ephemeral_messages_to_java(env, ephemeral_messages)?;
    let change_events_java = change_events_to_java(env, change_events)?;
    let peer_state_changes_java = empty_map(env)?;

    repo_bindings::DocActorResult::new(
        env,
        &io_tasks_java,
        &outgoing_java,
        &ephemeral_java,
        &change_events_java,
        stopped as jboolean,
        &peer_state_changes_java,
    )
}

fn doc_io_task_list_to_java<'local>(
    env: &mut jni::Env<'local>,
    tasks: Vec<IoTask<DocumentIoTask>>,
) -> jni::errors::Result<JList<'local>> {
    let list = ArrayList::new(env)?;
    for task in tasks {
        let task_id = repo_bindings::IoTaskId::new(env, u32::from(task.task_id) as i32)?;
        let action = doc_io_task_to_java(env, task.action)?;
        let action_obj: JObject = action.into();
        let iotask = repo_bindings::IoTask::new(env, &task_id, &action_obj)?;
        let iotask_obj: JObject = iotask.into();
        list.add(env, &iotask_obj)?;
    }
    let obj: JObject = list.into();
    JList::cast_local(env, obj)
}

fn doc_io_task_to_java<'local>(
    env: &mut jni::Env<'local>,
    task: DocumentIoTask,
) -> jni::errors::Result<repo_bindings::DocumentIoTask<'local>> {
    let obj: repo_bindings::DocumentIoTask<'local> = match task {
        DocumentIoTask::Storage(storage) => {
            let storage_java = storage_task_to_java(env, &storage)?;
            repo_bindings::DocumentIoTaskStorage::new(env, &storage_java)?.into()
        }
        DocumentIoTask::CheckAnnouncePolicy { peer_id } => {
            let pid_java = crate::repo::ids::peer_id_to_java(env, &peer_id)?;
            repo_bindings::DocumentIoTaskCheckAnnouncePolicy::new(env, &pid_java)?.into()
        }
    };
    Ok(obj)
}

fn doc_to_hub_msg_list_to_java<'local>(
    env: &mut jni::Env<'local>,
    msgs: Vec<DocToHubMsg>,
) -> jni::errors::Result<JList<'local>> {
    let list = ArrayList::new(env)?;
    for msg in msgs {
        let wrapper = unsafe { msg.store_as_pointer(env)? };
        let obj: JObject = wrapper.into();
        list.add(env, &obj)?;
    }
    let obj: JObject = list.into();
    JList::cast_local(env, obj)
}

fn ephemeral_messages_to_java<'local>(
    env: &mut jni::Env<'local>,
    msgs: Vec<Vec<u8>>,
) -> jni::errors::Result<JList<'local>> {
    let list = ArrayList::new(env)?;
    for msg in msgs {
        let jbytes = env.byte_array_from_slice(&msg)?;
        let obj: JObject = jbytes.into();
        list.add(env, &obj)?;
    }
    let obj: JObject = list.into();
    JList::cast_local(env, obj)
}

// --- IoResult<DocumentIoResult> (Java → Rust) ------------------------

fn document_io_result_from_java<'local>(
    env: &mut jni::Env<'local>,
    result: repo_bindings::IoResult<'local>,
) -> jni::errors::Result<IoResult<DocumentIoResult>> {
    let task_id = {
        let tid = result.task_id(env)?;
        IoTaskId::from(tid.value(env)? as u32)
    };
    let payload_obj = result.payload(env)?;
    let payload = document_io_result_payload_from_java(env, payload_obj)?;
    Ok(IoResult { task_id, payload })
}

fn document_io_result_payload_from_java<'local>(
    env: &mut jni::Env<'local>,
    obj: JObject<'local>,
) -> jni::errors::Result<DocumentIoResult> {
    use jni::refs::Reference;
    if env.is_instance_of(
        &obj,
        repo_bindings::DocumentIoResultStorage::class_name().as_ref(),
    )? {
        let storage = repo_bindings::DocumentIoResultStorage::cast_local(env, obj)?;
        let inner = storage.value0(env)?;
        let rust_storage = storage_result_from_java(env, inner.into())?;
        Ok(DocumentIoResult::Storage(rust_storage))
    } else if env.is_instance_of(
        &obj,
        repo_bindings::DocumentIoResultCheckAnnouncePolicy::class_name().as_ref(),
    )? {
        let cap = repo_bindings::DocumentIoResultCheckAnnouncePolicy::cast_local(env, obj)?;
        let flag = cap.value0(env)?;
        Ok(DocumentIoResult::CheckAnnouncePolicy(flag))
    } else {
        throw_illegal_argument(env, &JNIString::from("unknown DocumentIoResult variant"))?;

        Err(jni::errors::Error::JavaException)
    }
}

// --- Helpers used by hub.rs too ---

pub(crate) fn hub_io_task_list_to_java<'local>(
    env: &mut jni::Env<'local>,
    tasks: Vec<IoTask<HubIoAction>>,
) -> jni::errors::Result<JList<'local>> {
    let list = ArrayList::new(env)?;
    for task in tasks {
        let task_id = repo_bindings::IoTaskId::new(env, u32::from(task.task_id) as i32)?;
        let action = hub_io_action_to_java(env, task.action)?;
        let action_obj: JObject = action.into();
        let iotask = repo_bindings::IoTask::new(env, &task_id, &action_obj)?;
        let iotask_obj: JObject = iotask.into();
        list.add(env, &iotask_obj)?;
    }
    let obj: JObject = list.into();
    JList::cast_local(env, obj)
}

fn hub_io_action_to_java<'local>(
    env: &mut jni::Env<'local>,
    action: HubIoAction,
) -> jni::errors::Result<repo_bindings::HubIoAction<'local>> {
    let obj: repo_bindings::HubIoAction<'local> = match action {
        HubIoAction::Send { connection_id, msg } => {
            let cid = repo_bindings::ConnectionId::new(env, u32::from(connection_id) as i32)?;
            let bytes = env.byte_array_from_slice(&msg)?;
            repo_bindings::HubIoActionSend::new(env, &cid, &bytes)?.into()
        }
        HubIoAction::Disconnect { connection_id } => {
            let cid = repo_bindings::ConnectionId::new(env, u32::from(connection_id) as i32)?;
            repo_bindings::HubIoActionDisconnect::new(env, &cid)?.into()
        }
    };
    Ok(obj)
}

pub(crate) fn actor_message_list_to_java<'local>(
    env: &mut jni::Env<'local>,
    messages: Vec<(samod_core::DocumentActorId, HubToDocMsg)>,
) -> jni::errors::Result<JList<'local>> {
    let list = ArrayList::new(env)?;
    for (actor_id, msg) in messages {
        let aid = repo_bindings::DocumentActorId::new(env, u32::from(actor_id) as i32)?;
        let wrapper = unsafe { msg.store_as_pointer(env)? };
        let am = repo_bindings::ActorMessage::new(env, &aid, &wrapper)?;
        let obj: JObject = am.into();
        list.add(env, &obj)?;
    }
    let obj: JObject = list.into();
    JList::cast_local(env, obj)
}

fn change_events_to_java<'local>(
    env: &mut jni::Env<'local>,
    events: Vec<DocumentChanged>,
) -> jni::errors::Result<JList<'local>> {
    let list = ArrayList::new(env)?;
    for event in events {
        let heads_list = ArrayList::new(env)?;
        for hash in &event.new_heads {
            let jhash = crate::interop::changehash_to_jobject(env, hash)?;
            let obj: JObject = jhash.into();
            heads_list.add(env, &obj)?;
        }
        let heads_jlist = {
            let obj: JObject = heads_list.into();
            JList::cast_local(env, obj)?
        };
        let java_event = repo_bindings::DocumentChanged::new(env, &heads_jlist)?;
        let event_obj: JObject = java_event.into();
        list.add(env, &event_obj)?;
    }
    let obj: JObject = list.into();
    JList::cast_local(env, obj)
}

// --- Shared helpers ----

fn empty_map<'local>(
    env: &mut jni::Env<'local>,
) -> jni::errors::Result<jni::objects::JMap<'local>> {
    let map = JHashMap::new(env)?;
    let obj: JObject = map.into();
    jni::objects::JMap::cast_local(env, obj)
}

// Keep JByteArray in scope — used by HubIoAction::Send.
const _: fn(JByteArray<'_>) = |_| {};

// HubIoResult variants are unit-like; no conversion function needed yet.
// They surface as empty Java objects when and if samod emits HubIoResult.
const _: fn(HubIoResult) = |_| {};
