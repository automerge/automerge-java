//! `bind_java_type!` declarations for types in the `org.automerge.repo`
//! package.
//!
//! Kept separate from `crate::bindings` so the repo layer stays modular.

#![allow(dead_code)]

use jni::bind_java_type;

// AutomergeSys-parallel inner pointer classes on RepoSys.
bind_java_type! { pub HubPointer => org.automerge.repo.RepoSys::HubPointer }
bind_java_type! { pub DocumentActorPointer => org.automerge.repo.RepoSys::DocumentActorPointer }
bind_java_type! { pub SamodLoaderPointer => org.automerge.repo.RepoSys::SamodLoaderPointer }

// ID types ----------------------------------------------------------------

bind_java_type! {
    pub DocumentId => org.automerge.repo.DocumentId,
    constructors {
        fn new(bytes: jbyte[]),
    },
    fields { bytes: jbyte[] },
}

bind_java_type! {
    pub PeerId => org.automerge.repo.PeerId,
    constructors {
        fn new(value: JString),
    },
    fields { value: JString },
}

bind_java_type! {
    pub StorageId => org.automerge.repo.StorageId,
    constructors {
        fn new(value: JString),
    },
    fields { value: JString },
}

bind_java_type! {
    pub CommandId => org.automerge.repo.CommandId,
    constructors {
        fn new(value: jint),
    },
    fields { value: jint },
}

bind_java_type! {
    pub IoTaskId => org.automerge.repo.IoTaskId,
    constructors {
        fn new(value: jint),
    },
    fields { value: jint },
}

bind_java_type! {
    pub DocumentActorId => org.automerge.repo.DocumentActorId,
    constructors {
        fn new(value: jint),
    },
    fields { value: jint },
}

bind_java_type! {
    pub ConnectionId => org.automerge.repo.ConnectionId,
    constructors { fn new(value: jint) },
    fields { value: jint },
}

bind_java_type! {
    pub DialerId => org.automerge.repo.DialerId,
    constructors { fn new(value: jint) },
    fields { value: jint },
}

bind_java_type! {
    pub ListenerId => org.automerge.repo.ListenerId,
    constructors { fn new(value: jint) },
    fields { value: jint },
}

bind_java_type! {
    pub AutomergeUrl => org.automerge.repo.AutomergeUrl,
    type_map = { DocumentId => org.automerge.repo.DocumentId },
    constructors {
        fn new(doc_id: DocumentId),
    },
}

// Storage --------------------------------------------------------------

bind_java_type! {
    pub StorageKey => org.automerge.repo.StorageKey,
    constructors {
        fn new(parts: JString[]),
    },
    fields { parts: JString[] },
}

bind_java_type! { pub StorageResult => org.automerge.repo.StorageResult }

bind_java_type! {
    pub StorageResultLoad => org.automerge.repo.StorageResult::Load,
    type_map = { StorageResult => org.automerge.repo.StorageResult },
    is_instance_of = { base: StorageResult },
    constructors { fn new(value: jbyte[]) },
    fields { value: java.util.Optional },
}

bind_java_type! {
    pub StorageResultLoadRange => org.automerge.repo.StorageResult::LoadRange,
    type_map = { StorageResult => org.automerge.repo.StorageResult },
    is_instance_of = { base: StorageResult },
    constructors { fn new(values: java.util.Map) },
    fields { values: java.util.Map },
}

bind_java_type! {
    pub StorageResultPut => org.automerge.repo.StorageResult::Put,
    type_map = { StorageResult => org.automerge.repo.StorageResult },
    is_instance_of = { base: StorageResult },
    constructors { fn new() },
}

bind_java_type! {
    pub StorageResultDelete => org.automerge.repo.StorageResult::Delete,
    type_map = { StorageResult => org.automerge.repo.StorageResult },
    is_instance_of = { base: StorageResult },
    constructors { fn new() },
}

bind_java_type! { pub StorageTask => org.automerge.repo.StorageTask }

bind_java_type! {
    pub StorageTaskLoad => org.automerge.repo.StorageTask::Load,
    type_map = {
        StorageTask => org.automerge.repo.StorageTask,
        StorageKey  => org.automerge.repo.StorageKey,
    },
    is_instance_of = { base: StorageTask },
    constructors { fn new(key: StorageKey) },
    fields { key: StorageKey },
}

bind_java_type! {
    pub StorageTaskLoadRange => org.automerge.repo.StorageTask::LoadRange,
    type_map = {
        StorageTask => org.automerge.repo.StorageTask,
        StorageKey  => org.automerge.repo.StorageKey,
    },
    is_instance_of = { base: StorageTask },
    constructors { fn new(prefix: StorageKey) },
    fields { prefix: StorageKey },
}

bind_java_type! {
    pub StorageTaskPut => org.automerge.repo.StorageTask::Put,
    type_map = {
        StorageTask => org.automerge.repo.StorageTask,
        StorageKey  => org.automerge.repo.StorageKey,
    },
    is_instance_of = { base: StorageTask },
    constructors { fn new(key: StorageKey, value: jbyte[]) },
    fields {
        key: StorageKey,
        value: jbyte[],
    },
}

bind_java_type! {
    pub StorageTaskDelete => org.automerge.repo.StorageTask::Delete,
    type_map = {
        StorageTask => org.automerge.repo.StorageTask,
        StorageKey  => org.automerge.repo.StorageKey,
    },
    is_instance_of = { base: StorageTask },
    constructors { fn new(key: StorageKey) },
    fields { key: StorageKey },
}

// Hub results ----------------------------------------------------------

bind_java_type! {
    pub HubResults => org.automerge.repo.HubResults,
    constructors {
        #[expect(clippy::too_many_arguments)]
        fn new(
            new_tasks: java.util.List,
            completed_commands: java.util.Map,
            spawn_actors: java.util.List,
            actor_messages: java.util.List,
            connection_events: java.util.List,
            dial_requests: java.util.List,
            dialer_events: java.util.List,
            stopped: jboolean,
        ),
    },
}

bind_java_type! {
    pub HubCommandResult => org.automerge.repo.HubCommandResult,
    type_map = {
        CommandId  => org.automerge.repo.CommandId,
        HubResults => org.automerge.repo.HubResults,
    },
    constructors { fn new(command_id: CommandId, results: HubResults) },
}

// CommandResult hierarchy ----------------------------------------------

bind_java_type! { pub CommandResult => org.automerge.repo.CommandResult }

bind_java_type! {
    pub CommandResultCreateConnection => org.automerge.repo.CommandResult::CreateConnection,
    type_map = {
        CommandResult => org.automerge.repo.CommandResult,
        ConnectionId  => org.automerge.repo.ConnectionId,
    },
    is_instance_of = { base: CommandResult },
    constructors { fn new(connection_id: ConnectionId) },
}

bind_java_type! {
    pub CommandResultDisconnectConnection => org.automerge.repo.CommandResult::DisconnectConnection,
    type_map = { CommandResult => org.automerge.repo.CommandResult },
    is_instance_of = { base: CommandResult },
    constructors { fn new() },
}

bind_java_type! {
    pub CommandResultReceive => org.automerge.repo.CommandResult::Receive,
    type_map = {
        CommandResult => org.automerge.repo.CommandResult,
        ConnectionId  => org.automerge.repo.ConnectionId,
    },
    is_instance_of = { base: CommandResult },
    constructors { fn new(connection_id: ConnectionId, error: java.util.Optional) },
}

bind_java_type! {
    pub CommandResultActorReady => org.automerge.repo.CommandResult::ActorReady,
    type_map = { CommandResult => org.automerge.repo.CommandResult },
    is_instance_of = { base: CommandResult },
    constructors { fn new() },
}

bind_java_type! {
    pub CommandResultCreateDocument => org.automerge.repo.CommandResult::CreateDocument,
    type_map = {
        CommandResult     => org.automerge.repo.CommandResult,
        DocumentActorId   => org.automerge.repo.DocumentActorId,
        DocumentId        => org.automerge.repo.DocumentId,
    },
    is_instance_of = { base: CommandResult },
    constructors { fn new(actor_id: DocumentActorId, document_id: DocumentId) },
}

bind_java_type! {
    pub CommandResultFindDocument => org.automerge.repo.CommandResult::FindDocument,
    type_map = {
        CommandResult   => org.automerge.repo.CommandResult,
        DocumentActorId => org.automerge.repo.DocumentActorId,
    },
    is_instance_of = { base: CommandResult },
    constructors { fn new(actor_id: DocumentActorId, found: jboolean) },
}

bind_java_type! {
    pub CommandResultAddDialer => org.automerge.repo.CommandResult::AddDialer,
    type_map = {
        CommandResult => org.automerge.repo.CommandResult,
        DialerId      => org.automerge.repo.DialerId,
    },
    is_instance_of = { base: CommandResult },
    constructors { fn new(dialer_id: DialerId) },
}

bind_java_type! {
    pub CommandResultAddListener => org.automerge.repo.CommandResult::AddListener,
    type_map = {
        CommandResult => org.automerge.repo.CommandResult,
        ListenerId    => org.automerge.repo.ListenerId,
    },
    is_instance_of = { base: CommandResult },
    constructors { fn new(listener_id: ListenerId) },
}

// Document-actor messaging and IO ---------------------------------------

// Top-level pointer-bearing message classes.
bind_java_type! { pub HubToDocMsg => org.automerge.repo.HubToDocMsg }
bind_java_type! { pub DocToHubMsg => org.automerge.repo.DocToHubMsg }

bind_java_type! {
    pub ActorMessage => org.automerge.repo.ActorMessage,
    type_map = {
        DocumentActorId => org.automerge.repo.DocumentActorId,
        HubToDocMsg     => org.automerge.repo.HubToDocMsg,
    },
    constructors { fn new(actor_id: DocumentActorId, message: HubToDocMsg) },
}

// HubIoAction + HubIoResult ---------------------------------------------

bind_java_type! { pub HubIoAction => org.automerge.repo.HubIoAction }

bind_java_type! {
    pub HubIoActionSend => org.automerge.repo.HubIoAction::Send,
    type_map = {
        HubIoAction  => org.automerge.repo.HubIoAction,
        ConnectionId => org.automerge.repo.ConnectionId,
    },
    is_instance_of = { base: HubIoAction },
    constructors { fn new(connection_id: ConnectionId, message: jbyte[]) },
}

bind_java_type! {
    pub HubIoActionDisconnect => org.automerge.repo.HubIoAction::Disconnect,
    type_map = {
        HubIoAction  => org.automerge.repo.HubIoAction,
        ConnectionId => org.automerge.repo.ConnectionId,
    },
    is_instance_of = { base: HubIoAction },
    constructors { fn new(connection_id: ConnectionId) },
}

bind_java_type! { pub HubIoResult => org.automerge.repo.HubIoResult }

bind_java_type! {
    pub HubIoResultSend => org.automerge.repo.HubIoResult::Send,
    type_map = { HubIoResult => org.automerge.repo.HubIoResult },
    is_instance_of = { base: HubIoResult },
    constructors { fn new() },
}

bind_java_type! {
    pub HubIoResultDisconnect => org.automerge.repo.HubIoResult::Disconnect,
    type_map = { HubIoResult => org.automerge.repo.HubIoResult },
    is_instance_of = { base: HubIoResult },
    constructors { fn new() },
}

// DocumentIoTask + DocumentIoResult -------------------------------------

bind_java_type! { pub DocumentIoTask => org.automerge.repo.DocumentIoTask }

bind_java_type! {
    pub DocumentIoTaskStorage => org.automerge.repo.DocumentIoTask::Storage,
    type_map = {
        DocumentIoTask => org.automerge.repo.DocumentIoTask,
        StorageTask    => org.automerge.repo.StorageTask,
    },
    is_instance_of = { base: DocumentIoTask },
    constructors { fn new(value0: StorageTask) },
}

bind_java_type! {
    pub DocumentIoTaskCheckAnnouncePolicy => org.automerge.repo.DocumentIoTask::CheckAnnouncePolicy,
    type_map = {
        DocumentIoTask => org.automerge.repo.DocumentIoTask,
        PeerId         => org.automerge.repo.PeerId,
    },
    is_instance_of = { base: DocumentIoTask },
    constructors { fn new(peer_id: PeerId) },
}

bind_java_type! { pub DocumentIoResult => org.automerge.repo.DocumentIoResult }

bind_java_type! {
    pub DocumentIoResultStorage => org.automerge.repo.DocumentIoResult::Storage,
    type_map = {
        DocumentIoResult => org.automerge.repo.DocumentIoResult,
        StorageResult    => org.automerge.repo.StorageResult,
    },
    is_instance_of = { base: DocumentIoResult },
    constructors { fn new(value0: StorageResult) },
    fields { value0: StorageResult },
}

bind_java_type! {
    pub DocumentIoResultCheckAnnouncePolicy => org.automerge.repo.DocumentIoResult::CheckAnnouncePolicy,
    type_map = { DocumentIoResult => org.automerge.repo.DocumentIoResult },
    is_instance_of = { base: DocumentIoResult },
    constructors { fn new(value0: jboolean) },
    fields { value0: jboolean },
}

// DocumentChanged / DocumentActor / SpawnedActor / DocActorResult --------

bind_java_type! {
    pub DocumentChanged => org.automerge.repo.DocumentChanged,
    constructors { fn new(new_heads: java.util.List) },
}

bind_java_type! {
    pub DocumentActor => org.automerge.repo.DocumentActor,
    type_map = {
        DocumentActorPointer => org.automerge.repo.RepoSys::DocumentActorPointer,
        DocumentActorId      => org.automerge.repo.DocumentActorId,
        DocumentId           => org.automerge.repo.DocumentId,
    },
    constructors {
        fn new(
            pointer: DocumentActorPointer,
            actor_id: DocumentActorId,
            document_id: DocumentId,
        ),
    },
}

bind_java_type! {
    pub DocActorResult => org.automerge.repo.DocActorResult,
    constructors {
        fn new(
            io_tasks: java.util.List,
            outgoing_messages: java.util.List,
            ephemeral_messages: java.util.List,
            change_events: java.util.List,
            stopped: jboolean,
            peer_state_changes: java.util.Map,
        ),
    },
}

bind_java_type! {
    pub SpawnedActor => org.automerge.repo.SpawnedActor,
    type_map = {
        DocumentActor  => org.automerge.repo.DocumentActor,
        DocActorResult => org.automerge.repo.DocActorResult,
    },
    constructors { fn new(actor: DocumentActor, initial_result: DocActorResult) },
}

// WithDocResult<T>: T is erased to Object on the Java side.
bind_java_type! {
    pub WithDocResult => org.automerge.repo.WithDocResult,
    type_map = { DocActorResult => org.automerge.repo.DocActorResult },
    constructors { fn new(value: JObject, actor_result: DocActorResult) },
}

// LoaderStepResult enum hierarchy --------------------------------------

bind_java_type! { pub LoaderStepResult => org.automerge.repo.LoaderStepResult }

bind_java_type! {
    pub LoaderStepResultNeedIo => org.automerge.repo.LoaderStepResult::NeedIo,
    type_map = { LoaderStepResult => org.automerge.repo.LoaderStepResult },
    is_instance_of = { base: LoaderStepResult },
    constructors { fn new(value0: java.util.List) },
}

bind_java_type! {
    pub LoaderStepResultLoaded => org.automerge.repo.LoaderStepResult::Loaded,
    type_map = {
        LoaderStepResult => org.automerge.repo.LoaderStepResult,
        HubPointer => org.automerge.repo.RepoSys::HubPointer,
    },
    is_instance_of = { base: LoaderStepResult },
    constructors { fn new(value0: HubPointer) },
}

// Dialer / Listener config ---------------------------------------------

bind_java_type! {
    pub BackoffConfig => org.automerge.repo.BackoffConfig,
    fields {
        initial_delay_ms: jlong,
        max_delay_ms: jlong,
        max_retries: java.util.Optional,
    },
}

bind_java_type! {
    pub DialerConfig => org.automerge.repo.DialerConfig,
    type_map = { BackoffConfig => org.automerge.repo.BackoffConfig },
    fields {
        backoff: BackoffConfig,
    },
}

bind_java_type! {
    pub ListenerConfig => org.automerge.repo.ListenerConfig,
    fields {
        url: JString,
    },
}

// Connection / Peer / Dial networking types ----------------------------

bind_java_type! { pub ConnectionOwner => org.automerge.repo.ConnectionOwner }

bind_java_type! {
    pub ConnectionOwnerDialer => org.automerge.repo.ConnectionOwner::DialerOwner,
    type_map = {
        ConnectionOwner => org.automerge.repo.ConnectionOwner,
        DialerId        => org.automerge.repo.DialerId,
    },
    is_instance_of = { base: ConnectionOwner },
    constructors { fn new(dialer_id: DialerId) },
}

bind_java_type! {
    pub ConnectionOwnerListener => org.automerge.repo.ConnectionOwner::ListenerOwner,
    type_map = {
        ConnectionOwner => org.automerge.repo.ConnectionOwner,
        ListenerId      => org.automerge.repo.ListenerId,
    },
    is_instance_of = { base: ConnectionOwner },
    constructors { fn new(listener_id: ListenerId) },
}

bind_java_type! {
    pub PeerInfo => org.automerge.repo.PeerInfo,
    type_map = { PeerId => org.automerge.repo.PeerId },
    constructors {
        fn new(peer_id: PeerId, metadata: java.util.Optional, protocol_version: JString),
    },
}

bind_java_type! { pub ConnectionEvent => org.automerge.repo.ConnectionEvent }

bind_java_type! {
    pub ConnectionEventHandshakeCompleted => org.automerge.repo.ConnectionEvent::HandshakeCompleted,
    type_map = {
        ConnectionEvent => org.automerge.repo.ConnectionEvent,
        ConnectionId    => org.automerge.repo.ConnectionId,
        ConnectionOwner => org.automerge.repo.ConnectionOwner,
        PeerInfo        => org.automerge.repo.PeerInfo,
    },
    is_instance_of = { base: ConnectionEvent },
    constructors { fn new(connection_id: ConnectionId, owner: ConnectionOwner, peer_info: PeerInfo) },
}

bind_java_type! {
    pub ConnectionEventConnectionFailed => org.automerge.repo.ConnectionEvent::ConnectionFailed,
    type_map = {
        ConnectionEvent => org.automerge.repo.ConnectionEvent,
        ConnectionId    => org.automerge.repo.ConnectionId,
        ConnectionOwner => org.automerge.repo.ConnectionOwner,
    },
    is_instance_of = { base: ConnectionEvent },
    constructors { fn new(connection_id: ConnectionId, owner: ConnectionOwner, error: JString) },
}

bind_java_type! {
    pub DialRequest => org.automerge.repo.DialRequest,
    type_map = { DialerId => org.automerge.repo.DialerId },
    constructors { fn new(dialer_id: DialerId, url: JString) },
}

bind_java_type! { pub DialerEvent => org.automerge.repo.DialerEvent }

bind_java_type! {
    pub DialerEventMaxRetriesReached => org.automerge.repo.DialerEvent::MaxRetriesReached,
    type_map = {
        DialerEvent => org.automerge.repo.DialerEvent,
        DialerId    => org.automerge.repo.DialerId,
    },
    is_instance_of = { base: DialerEvent },
    constructors { fn new(dialer_id: DialerId, url: JString) },
}

// Generic IO wrappers --------------------------------------------------

bind_java_type! {
    pub IoTask => org.automerge.repo.IoTask,
    type_map = { IoTaskId => org.automerge.repo.IoTaskId },
    constructors { fn new(task_id: IoTaskId, action: JObject) },
    fields {
        task_id: IoTaskId,
        action: JObject,
    },
}

bind_java_type! {
    pub IoResult => org.automerge.repo.IoResult,
    type_map = { IoTaskId => org.automerge.repo.IoTaskId },
    constructors { fn new(task_id: IoTaskId, payload: JObject) },
    fields {
        task_id: IoTaskId,
        payload: JObject,
    },
}
