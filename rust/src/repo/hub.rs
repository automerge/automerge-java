//! Native methods on `RepoSys` that drive the samod `Hub` — per-event
//! combined functions that both create the event and call `hub.handle_event()`
//! in a single JNI call.

use jni::{
    objects::{JByteArray, JClass, JList, JObject, JString},
    refs::Reference,
    strings::JNIString,
    sys::{jboolean, jlong},
    NativeMethod,
};
use samod_core::actors::document::{DocumentActor, SpawnArgs};
use samod_core::actors::hub::io::HubIoResult;
use samod_core::actors::hub::{CommandId, CommandResult, DispatchedCommand, Hub, HubEvent};
use samod_core::actors::DocToHubMsg;
use samod_core::io::{IoResult, IoTaskId};
use samod_core::UnixTimestamp;
use samod_core::{
    network::{ConnectionEvent, PeerInfo},
    BackoffConfig, ConnectionId, ConnectionOwner, DialRequest, DialerConfig, DialerEvent, DialerId,
    ListenerConfig, ListenerId,
};
use url::Url;

use crate::interop::JavaPointer;
use crate::repo::bindings as repo_bindings;
use crate::repo::document_actor::{actor_message_list_to_java, hub_io_task_list_to_java};
use crate::repo::ids::document_id_from_java;
use crate::{
    bindings::{ArrayList, HashMap as JHashMap},
    interop::throw_illegal_argument,
};

const _METHODS: &[NativeMethod] = &[
    // Pointer lifecycle
    repo_native! { static extern fn free_hub(hub: repo_bindings::HubPointer) },
    // Fire-and-forget events → HubResults
    repo_native! { static extern fn hub_handle_event_tick(
        hub: repo_bindings::HubPointer,
        timestamp: jlong
    ) -> repo_bindings::HubResults },
    repo_native! { static extern fn hub_handle_event_stop(
        hub: repo_bindings::HubPointer,
        timestamp: jlong
    ) -> repo_bindings::HubResults },
    repo_native! { static extern fn hub_handle_event_actor_message(
        hub: repo_bindings::HubPointer,
        timestamp: jlong,
        actor_id: repo_bindings::DocumentActorId,
        message: repo_bindings::DocToHubMsg
    ) -> repo_bindings::HubResults },
    repo_native! { static extern fn hub_handle_event_io_complete(
        hub: repo_bindings::HubPointer,
        timestamp: jlong,
        result: repo_bindings::IoResult
    ) -> repo_bindings::HubResults },
    repo_native! { static extern fn hub_handle_event_connection_lost(
        hub: repo_bindings::HubPointer,
        timestamp: jlong,
        connection_id: repo_bindings::ConnectionId
    ) -> repo_bindings::HubResults },
    repo_native! { static extern fn hub_handle_event_dial_failed(
        hub: repo_bindings::HubPointer,
        timestamp: jlong,
        dialer_id: repo_bindings::DialerId,
        error: JString
    ) -> repo_bindings::HubResults },
    repo_native! { static extern fn hub_handle_event_remove_dialer(
        hub: repo_bindings::HubPointer,
        timestamp: jlong,
        dialer_id: repo_bindings::DialerId
    ) -> repo_bindings::HubResults },
    repo_native! { static extern fn hub_handle_event_remove_listener(
        hub: repo_bindings::HubPointer,
        timestamp: jlong,
        listener_id: repo_bindings::ListenerId
    ) -> repo_bindings::HubResults },
    // Command events → HubCommandResult
    repo_native! { static extern fn hub_handle_event_create_document(
        hub: repo_bindings::HubPointer,
        timestamp: jlong,
        initial_content: jbyte[]
    ) -> repo_bindings::HubCommandResult },
    repo_native! { static extern fn hub_handle_event_find_document(
        hub: repo_bindings::HubPointer,
        timestamp: jlong,
        document_id: repo_bindings::DocumentId
    ) -> repo_bindings::HubCommandResult },
    repo_native! { static extern fn hub_handle_event_add_dialer(
        hub: repo_bindings::HubPointer,
        timestamp: jlong,
        config: repo_bindings::DialerConfig,
        url: JString,
    ) -> repo_bindings::HubCommandResult },
    repo_native! { static extern fn hub_handle_event_add_listener(
        hub: repo_bindings::HubPointer,
        timestamp: jlong,
        config: repo_bindings::ListenerConfig
    ) -> repo_bindings::HubCommandResult },
    repo_native! { static extern fn hub_handle_event_create_dialer_connection(
        hub: repo_bindings::HubPointer,
        timestamp: jlong,
        dialer_id: repo_bindings::DialerId
    ) -> repo_bindings::HubCommandResult },
    repo_native! { static extern fn hub_handle_event_create_listener_connection(
        hub: repo_bindings::HubPointer,
        timestamp: jlong,
        listener_id: repo_bindings::ListenerId
    ) -> repo_bindings::HubCommandResult },
    repo_native! { static extern fn hub_handle_event_receive(
        hub: repo_bindings::HubPointer,
        timestamp: jlong,
        connection_id: repo_bindings::ConnectionId,
        message: jbyte[]
    ) -> repo_bindings::HubCommandResult },
    // Hub status
    repo_native! { static extern fn hub_is_stopped(hub: repo_bindings::HubPointer) -> jboolean },
    repo_native! { static extern fn hub_get_peer_id(hub: repo_bindings::HubPointer) -> repo_bindings::PeerId },
    repo_native! { static extern fn hub_get_storage_id(hub: repo_bindings::HubPointer) -> repo_bindings::StorageId },
];

fn free_hub<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    hub: repo_bindings::HubPointer<'local>,
) -> jni::errors::Result<()> {
    let _hub = unsafe { Hub::take_from_pointer(env, hub)? };
    Ok(())
}

// --- Fire-and-forget events -------------------------------------------

fn hub_handle_event_tick<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    hub: repo_bindings::HubPointer<'local>,
    timestamp: jlong,
) -> jni::errors::Result<repo_bindings::HubResults<'local>> {
    let ts = UnixTimestamp::from_millis(timestamp as u128);
    let results = {
        let mut h = unsafe { Hub::borrow_from_pointer(env, hub)? };
        h.handle_event(&mut rand::rng(), ts, HubEvent::tick())
    };
    hub_results_to_java(env, ts, results)
}

fn hub_handle_event_stop<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    hub: repo_bindings::HubPointer<'local>,
    timestamp: jlong,
) -> jni::errors::Result<repo_bindings::HubResults<'local>> {
    let ts = UnixTimestamp::from_millis(timestamp as u128);
    let results = {
        let mut h = unsafe { Hub::borrow_from_pointer(env, hub)? };
        h.handle_event(&mut rand::rng(), ts, HubEvent::stop())
    };
    hub_results_to_java(env, ts, results)
}

fn hub_handle_event_actor_message<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    hub: repo_bindings::HubPointer<'local>,
    timestamp: jlong,
    actor_id: repo_bindings::DocumentActorId<'local>,
    message: repo_bindings::DocToHubMsg<'local>,
) -> jni::errors::Result<repo_bindings::HubResults<'local>> {
    let actor_id = samod_core::DocumentActorId::from(actor_id.value(env)? as u32);
    let message = unsafe { DocToHubMsg::take_from_pointer(env, message)? };
    let ts = UnixTimestamp::from_millis(timestamp as u128);
    let results = {
        let mut h = unsafe { Hub::borrow_from_pointer(env, hub)? };
        h.handle_event(
            &mut rand::rng(),
            ts,
            HubEvent::actor_message(actor_id, message),
        )
    };
    hub_results_to_java(env, ts, results)
}

fn hub_handle_event_io_complete<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    hub: repo_bindings::HubPointer<'local>,
    timestamp: jlong,
    result: repo_bindings::IoResult<'local>,
) -> jni::errors::Result<repo_bindings::HubResults<'local>> {
    let task_id = {
        let tid = result.task_id(env)?;
        IoTaskId::from(tid.value(env)? as u32)
    };
    let payload_obj = result.payload(env)?;
    let payload = hub_io_result_from_java(env, payload_obj)?;
    let io_result = IoResult { task_id, payload };
    let ts = UnixTimestamp::from_millis(timestamp as u128);
    let results = {
        let mut h = unsafe { Hub::borrow_from_pointer(env, hub)? };
        h.handle_event(&mut rand::rng(), ts, HubEvent::io_complete(io_result))
    };
    hub_results_to_java(env, ts, results)
}

fn hub_handle_event_connection_lost<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    hub: repo_bindings::HubPointer<'local>,
    timestamp: jlong,
    connection_id: repo_bindings::ConnectionId<'local>,
) -> jni::errors::Result<repo_bindings::HubResults<'local>> {
    let cid = ConnectionId::from(connection_id.value(env)? as u32);
    let ts = UnixTimestamp::from_millis(timestamp as u128);
    let results = {
        let mut h = unsafe { Hub::borrow_from_pointer(env, hub)? };
        h.handle_event(&mut rand::rng(), ts, HubEvent::connection_lost(cid))
    };
    hub_results_to_java(env, ts, results)
}

fn hub_handle_event_dial_failed<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    hub: repo_bindings::HubPointer<'local>,
    timestamp: jlong,
    dialer_id: repo_bindings::DialerId<'local>,
    error: JString<'local>,
) -> jni::errors::Result<repo_bindings::HubResults<'local>> {
    let did = DialerId::from(dialer_id.value(env)? as u32);
    let error_str = error.to_string();
    let ts = UnixTimestamp::from_millis(timestamp as u128);
    let results = {
        let mut h = unsafe { Hub::borrow_from_pointer(env, hub)? };
        h.handle_event(&mut rand::rng(), ts, HubEvent::dial_failed(did, error_str))
    };
    hub_results_to_java(env, ts, results)
}

fn hub_handle_event_remove_dialer<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    hub: repo_bindings::HubPointer<'local>,
    timestamp: jlong,
    dialer_id: repo_bindings::DialerId<'local>,
) -> jni::errors::Result<repo_bindings::HubResults<'local>> {
    let did = DialerId::from(dialer_id.value(env)? as u32);
    let ts = UnixTimestamp::from_millis(timestamp as u128);
    let results = {
        let mut h = unsafe { Hub::borrow_from_pointer(env, hub)? };
        h.handle_event(&mut rand::rng(), ts, HubEvent::remove_dialer(did))
    };
    hub_results_to_java(env, ts, results)
}

fn hub_handle_event_remove_listener<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    hub: repo_bindings::HubPointer<'local>,
    timestamp: jlong,
    listener_id: repo_bindings::ListenerId<'local>,
) -> jni::errors::Result<repo_bindings::HubResults<'local>> {
    let lid = ListenerId::from(listener_id.value(env)? as u32);
    let ts = UnixTimestamp::from_millis(timestamp as u128);
    let results = {
        let mut h = unsafe { Hub::borrow_from_pointer(env, hub)? };
        h.handle_event(&mut rand::rng(), ts, HubEvent::remove_listener(lid))
    };
    hub_results_to_java(env, ts, results)
}

// --- Command events ---------------------------------------------------

fn hub_handle_event_create_document<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    hub: repo_bindings::HubPointer<'local>,
    timestamp: jlong,
    initial_content: JByteArray<'local>,
) -> jni::errors::Result<repo_bindings::HubCommandResult<'local>> {
    let bytes = env.convert_byte_array(&initial_content)?;
    let content = match automerge::Automerge::load(&bytes) {
        Ok(doc) => doc,
        Err(e) => {
            throw_illegal_argument(
                env,
                &JNIString::from(format!("invalid initial Automerge content: {}", e)),
            )?;

            return Err(jni::errors::Error::JavaException);
        }
    };
    let dispatched = HubEvent::create_document(content);
    handle_dispatched_command(env, hub, timestamp, dispatched)
}

fn hub_handle_event_find_document<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    hub: repo_bindings::HubPointer<'local>,
    timestamp: jlong,
    document_id: repo_bindings::DocumentId<'local>,
) -> jni::errors::Result<repo_bindings::HubCommandResult<'local>> {
    let doc_id = document_id_from_java(env, &document_id)?;
    handle_dispatched_command(env, hub, timestamp, HubEvent::find_document(doc_id))
}

fn hub_handle_event_add_dialer<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    hub: repo_bindings::HubPointer<'local>,
    timestamp: jlong,
    config: repo_bindings::DialerConfig<'local>,
    url: JString<'local>,
) -> jni::errors::Result<repo_bindings::HubCommandResult<'local>> {
    let dialer_config = dialer_config_from_java(env, config, url)?;
    handle_dispatched_command(env, hub, timestamp, HubEvent::add_dialer(dialer_config))
}

fn hub_handle_event_add_listener<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    hub: repo_bindings::HubPointer<'local>,
    timestamp: jlong,
    config: repo_bindings::ListenerConfig<'local>,
) -> jni::errors::Result<repo_bindings::HubCommandResult<'local>> {
    let listener_config = listener_config_from_java(env, config)?;
    handle_dispatched_command(env, hub, timestamp, HubEvent::add_listener(listener_config))
}

fn hub_handle_event_create_dialer_connection<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    hub: repo_bindings::HubPointer<'local>,
    timestamp: jlong,
    dialer_id: repo_bindings::DialerId<'local>,
) -> jni::errors::Result<repo_bindings::HubCommandResult<'local>> {
    let did = DialerId::from(dialer_id.value(env)? as u32);
    handle_dispatched_command(env, hub, timestamp, HubEvent::create_dialer_connection(did))
}

fn hub_handle_event_create_listener_connection<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    hub: repo_bindings::HubPointer<'local>,
    timestamp: jlong,
    listener_id: repo_bindings::ListenerId<'local>,
) -> jni::errors::Result<repo_bindings::HubCommandResult<'local>> {
    let lid = ListenerId::from(listener_id.value(env)? as u32);
    handle_dispatched_command(
        env,
        hub,
        timestamp,
        HubEvent::create_listener_connection(lid),
    )
}

fn hub_handle_event_receive<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    hub: repo_bindings::HubPointer<'local>,
    timestamp: jlong,
    connection_id: repo_bindings::ConnectionId<'local>,
    message: JByteArray<'local>,
) -> jni::errors::Result<repo_bindings::HubCommandResult<'local>> {
    let cid = ConnectionId::from(connection_id.value(env)? as u32);
    let msg = env.convert_byte_array(&message)?;
    handle_dispatched_command(env, hub, timestamp, HubEvent::receive(cid, msg))
}

// --- Hub status methods -----------------------------------------------

fn hub_is_stopped<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    hub: repo_bindings::HubPointer<'local>,
) -> jni::errors::Result<jboolean> {
    let hub_guard = unsafe { Hub::borrow_from_pointer(env, hub)? };
    Ok(hub_guard.is_stopped() as jboolean)
}

fn hub_get_peer_id<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    hub: repo_bindings::HubPointer<'local>,
) -> jni::errors::Result<repo_bindings::PeerId<'local>> {
    let peer_id = {
        let hub_guard = unsafe { Hub::borrow_from_pointer(env, hub)? };
        hub_guard.peer_id()
    };
    crate::repo::ids::peer_id_to_java(env, &peer_id)
}

fn hub_get_storage_id<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    hub: repo_bindings::HubPointer<'local>,
) -> jni::errors::Result<repo_bindings::StorageId<'local>> {
    let storage_id = {
        let hub_guard = unsafe { Hub::borrow_from_pointer(env, hub)? };
        hub_guard.storage_id()
    };
    let jstr = env.new_string(storage_id.to_string())?;
    repo_bindings::StorageId::new(env, &jstr)
}

// --- Helpers ----------------------------------------------------------

/// Handle a `DispatchedCommand`: call hub.handle_event() and return a
/// `HubCommandResult` combining the CommandId with the HubResults.
fn handle_dispatched_command<'local>(
    env: &mut jni::Env<'local>,
    hub: repo_bindings::HubPointer<'local>,
    timestamp: jlong,
    dispatched: DispatchedCommand,
) -> jni::errors::Result<repo_bindings::HubCommandResult<'local>> {
    let ts = UnixTimestamp::from_millis(timestamp as u128);
    let DispatchedCommand { command_id, event } = dispatched;
    let results = {
        let mut h = unsafe { Hub::borrow_from_pointer(env, hub)? };
        h.handle_event(&mut rand::rng(), ts, event)
    };
    let hub_results = hub_results_to_java(env, ts, results)?;
    let cmd_id = command_id_to_java(env, command_id)?;
    repo_bindings::HubCommandResult::new(env, &cmd_id, &hub_results)
}

fn hub_io_result_from_java<'local>(
    env: &mut jni::Env<'local>,
    obj: JObject<'local>,
) -> jni::errors::Result<HubIoResult> {
    if env.is_instance_of(&obj, repo_bindings::HubIoResultSend::class_name().as_ref())? {
        Ok(HubIoResult::Send)
    } else if env.is_instance_of(
        &obj,
        repo_bindings::HubIoResultDisconnect::class_name().as_ref(),
    )? {
        Ok(HubIoResult::Disconnect)
    } else {
        throw_illegal_argument(env, &JNIString::from("unknown HubIoResult variant"))?;

        Err(jni::errors::Error::JavaException)
    }
}

fn dialer_config_from_java<'local>(
    env: &mut jni::Env<'local>,
    config: repo_bindings::DialerConfig<'local>,
    url_jstr: JString<'local>,
) -> jni::errors::Result<DialerConfig> {
    let url = url_jstr.to_string().parse::<Url>().map_err(|e| {
        let _ = throw_illegal_argument(env, &JNIString::from(format!("invalid dialer URL: {}", e)));
        jni::errors::Error::JavaException
    })?;
    let backoff = config.backoff(env)?;
    let initial_ms = backoff.initial_delay_ms(env)?;
    let max_ms = backoff.max_delay_ms(env)?;
    let max_retries_obj = backoff.max_retries(env)?;
    let opt = crate::bindings::Optional::cast_local(env, max_retries_obj)?;
    let max_retries = if opt.is_present(env)? {
        let val_obj = opt.get(env)?;
        let n = env
            .call_method(
                &val_obj,
                jni::jni_str!("intValue"),
                jni::jni_sig!("()I"),
                &[],
            )?
            .i()?;
        Some(n as u32)
    } else {
        None
    };
    Ok(DialerConfig {
        url,
        backoff: BackoffConfig {
            initial_delay: std::time::Duration::from_millis(initial_ms as u64),
            max_delay: std::time::Duration::from_millis(max_ms as u64),
            max_retries,
        },
    })
}

fn listener_config_from_java<'local>(
    env: &mut jni::Env<'local>,
    config: repo_bindings::ListenerConfig<'local>,
) -> jni::errors::Result<ListenerConfig> {
    let url_jstr = config.url(env)?;
    let url = url_jstr.to_string().parse::<Url>().map_err(|e| {
        let _ = throw_illegal_argument(
            env,
            &JNIString::from(format!("invalid listener URL: {}", e)),
        );

        jni::errors::Error::JavaException
    })?;
    Ok(ListenerConfig { url })
}

pub(crate) fn command_id_to_java<'local>(
    env: &mut jni::Env<'local>,
    id: CommandId,
) -> jni::errors::Result<repo_bindings::CommandId<'local>> {
    repo_bindings::CommandId::new(env, u32::from(id) as i32)
}

/// Convert a samod-core `HubResults` into its Java counterpart.
/// `timestamp` is forwarded to `DocumentActor::new` for each spawned actor.
fn hub_results_to_java<'local>(
    env: &mut jni::Env<'local>,
    timestamp: UnixTimestamp,
    results: samod_core::actors::hub::HubResults,
) -> jni::errors::Result<repo_bindings::HubResults<'local>> {
    let samod_core::actors::hub::HubResults {
        new_tasks,
        completed_commands,
        spawn_actors,
        actor_messages,
        stopped,
        connection_events,
        dial_requests,
        dialer_events,
        ..
    } = results;

    let new_tasks = hub_io_task_list_to_java(env, new_tasks)?;
    let actor_messages = actor_message_list_to_java(env, actor_messages)?;
    let connection_events = connection_events_to_java(env, connection_events)?;
    let dial_requests = dial_requests_to_java(env, dial_requests)?;
    let dialer_events = dialer_events_to_java(env, dialer_events)?;

    let completed_commands = completed_commands_to_java(env, completed_commands)?;
    let spawn_actors = spawn_actors_to_java(env, timestamp, spawn_actors)?;

    repo_bindings::HubResults::new(
        env,
        &new_tasks,
        &completed_commands,
        &spawn_actors,
        &actor_messages,
        &connection_events,
        &dial_requests,
        &dialer_events,
        stopped as jboolean,
    )
}

fn completed_commands_to_java<'local>(
    env: &mut jni::Env<'local>,
    map: std::collections::HashMap<CommandId, CommandResult>,
) -> jni::errors::Result<jni::objects::JMap<'local>> {
    let jmap = JHashMap::new(env)?;
    for (cmd_id, cmd_result) in map {
        let key = command_id_to_java(env, cmd_id)?;
        let key_obj: JObject = key.into();
        let value = command_result_to_java(env, cmd_result)?;
        let value_obj: JObject = value.into();
        jmap.put(env, &key_obj, &value_obj)?;
    }
    let obj: JObject = jmap.into();
    jni::objects::JMap::cast_local(env, obj)
}

fn spawn_actors_to_java<'local>(
    env: &mut jni::Env<'local>,
    timestamp: UnixTimestamp,
    actors: Vec<SpawnArgs>,
) -> jni::errors::Result<JList<'local>> {
    let list = ArrayList::new(env)?;
    for args in actors {
        let actor_id = args.actor_id();
        let document_id = args.document_id().clone();
        let (actor, initial) = DocumentActor::new(timestamp, args);

        let actor_ptr = unsafe { actor.store_as_pointer(env)? };
        let actor_id_java = repo_bindings::DocumentActorId::new(env, u32::from(actor_id) as i32)?;
        let document_id_java = crate::repo::ids::document_id_to_java(env, &document_id)?;
        let java_actor =
            repo_bindings::DocumentActor::new(env, &actor_ptr, &actor_id_java, &document_id_java)?;

        let initial_java = crate::repo::document_actor::doc_actor_result_to_java(env, initial)?;
        let spawned = repo_bindings::SpawnedActor::new(env, &java_actor, &initial_java)?;
        let obj: JObject = spawned.into();
        list.add(env, &obj)?;
    }
    let obj: JObject = list.into();
    JList::cast_local(env, obj)
}

fn command_result_to_java<'local>(
    env: &mut jni::Env<'local>,
    result: CommandResult,
) -> jni::errors::Result<repo_bindings::CommandResult<'local>> {
    let obj: repo_bindings::CommandResult<'local> = match result {
        CommandResult::CreateConnection { connection_id } => {
            let cid = repo_bindings::ConnectionId::new(env, u32::from(connection_id) as i32)?;
            repo_bindings::CommandResultCreateConnection::new(env, &cid)?.into()
        }
        CommandResult::DisconnectConnection => {
            repo_bindings::CommandResultDisconnectConnection::new(env)?.into()
        }
        CommandResult::Receive {
            connection_id,
            error,
        } => {
            let cid = repo_bindings::ConnectionId::new(env, u32::from(connection_id) as i32)?;
            let err_opt = match error {
                Some(msg) => {
                    let jstr = env.new_string(msg)?;
                    let jstr_obj: JObject = jstr.into();
                    crate::bindings::Optional::of(env, &jstr_obj)?
                }
                None => crate::bindings::Optional::empty(env)?,
            };
            repo_bindings::CommandResultReceive::new(env, &cid, &err_opt)?.into()
        }
        CommandResult::ActorReady => repo_bindings::CommandResultActorReady::new(env)?.into(),
        CommandResult::CreateDocument {
            actor_id,
            document_id,
        } => {
            let aid = repo_bindings::DocumentActorId::new(env, u32::from(actor_id) as i32)?;
            let did = crate::repo::ids::document_id_to_java(env, &document_id)?;
            repo_bindings::CommandResultCreateDocument::new(env, &aid, &did)?.into()
        }
        CommandResult::FindDocument { actor_id, found } => {
            let aid = repo_bindings::DocumentActorId::new(env, u32::from(actor_id) as i32)?;
            repo_bindings::CommandResultFindDocument::new(env, &aid, found as jboolean)?.into()
        }
        CommandResult::AddDialer { dialer_id } => {
            let did = repo_bindings::DialerId::new(env, u32::from(dialer_id) as i32)?;
            repo_bindings::CommandResultAddDialer::new(env, &did)?.into()
        }
        CommandResult::AddListener { listener_id } => {
            let lid = repo_bindings::ListenerId::new(env, u32::from(listener_id) as i32)?;
            repo_bindings::CommandResultAddListener::new(env, &lid)?.into()
        }
    };
    Ok(obj)
}

fn connection_events_to_java<'local>(
    env: &mut jni::Env<'local>,
    events: Vec<ConnectionEvent>,
) -> jni::errors::Result<JList<'local>> {
    let list = ArrayList::new(env)?;
    for event in events {
        let java_event: JObject = match event {
            ConnectionEvent::HandshakeCompleted {
                connection_id,
                owner,
                peer_info,
            } => {
                let cid = repo_bindings::ConnectionId::new(env, u32::from(connection_id) as i32)?;
                let jowner = connection_owner_to_java(env, owner)?;
                let jpeer = peer_info_to_java(env, peer_info)?;
                repo_bindings::ConnectionEventHandshakeCompleted::new(env, &cid, &jowner, &jpeer)?
                    .into()
            }
            ConnectionEvent::ConnectionFailed {
                connection_id,
                owner,
                error,
            } => {
                let cid = repo_bindings::ConnectionId::new(env, u32::from(connection_id) as i32)?;
                let jowner = connection_owner_to_java(env, owner)?;
                let jerr = env.new_string(&error)?;
                repo_bindings::ConnectionEventConnectionFailed::new(env, &cid, &jowner, &jerr)?
                    .into()
            }
            // StateChanged is not handled by Java's RepoRuntime (falls through in
            // processConnectionEvents). Skip it.
            ConnectionEvent::StateChanged { .. } => continue,
        };
        list.add(env, &java_event)?;
    }
    let obj: JObject = list.into();
    JList::cast_local(env, obj)
}

fn connection_owner_to_java<'local>(
    env: &mut jni::Env<'local>,
    owner: ConnectionOwner,
) -> jni::errors::Result<repo_bindings::ConnectionOwner<'local>> {
    match owner {
        ConnectionOwner::Dialer(dialer_id) => {
            let did = repo_bindings::DialerId::new(env, u32::from(dialer_id) as i32)?;
            repo_bindings::ConnectionOwnerDialer::new(env, &did).map(Into::into)
        }
        ConnectionOwner::Listener(listener_id) => {
            let lid = repo_bindings::ListenerId::new(env, u32::from(listener_id) as i32)?;
            repo_bindings::ConnectionOwnerListener::new(env, &lid).map(Into::into)
        }
    }
}

fn peer_info_to_java<'local>(
    env: &mut jni::Env<'local>,
    info: PeerInfo,
) -> jni::errors::Result<repo_bindings::PeerInfo<'local>> {
    let peer_id = crate::repo::ids::peer_id_to_java(env, &info.peer_id)?;
    let metadata = crate::bindings::Optional::empty(env)?;
    let metadata_obj: JObject = metadata.into();
    let protocol_version = env.new_string(&info.protocol_version)?;
    repo_bindings::PeerInfo::new(env, &peer_id, &metadata_obj, &protocol_version)
}

fn dial_requests_to_java<'local>(
    env: &mut jni::Env<'local>,
    requests: Vec<DialRequest>,
) -> jni::errors::Result<JList<'local>> {
    let list = ArrayList::new(env)?;
    for req in requests {
        let did = repo_bindings::DialerId::new(env, u32::from(req.dialer_id) as i32)?;
        let url = env.new_string(req.url.as_str())?;
        let java_req = repo_bindings::DialRequest::new(env, &did, &url)?;
        let obj: JObject = java_req.into();
        list.add(env, &obj)?;
    }
    let obj: JObject = list.into();
    JList::cast_local(env, obj)
}

fn dialer_events_to_java<'local>(
    env: &mut jni::Env<'local>,
    events: Vec<DialerEvent>,
) -> jni::errors::Result<JList<'local>> {
    let list = ArrayList::new(env)?;
    for event in events {
        let java_event: JObject = match event {
            DialerEvent::MaxRetriesReached { dialer_id, url } => {
                let did = repo_bindings::DialerId::new(env, u32::from(dialer_id) as i32)?;
                let jurl = env.new_string(url.as_str())?;
                repo_bindings::DialerEventMaxRetriesReached::new(env, &did, &jurl)?.into()
            }
        };
        list.add(env, &java_event)?;
    }
    let obj: JObject = list.into();
    JList::cast_local(env, obj)
}

// Keep JString in scope — used indirectly via some bindings' call paths —
// so the import isn't flagged unused when features are conditionally off.
const _: fn(JString<'_>) = |_| {};
