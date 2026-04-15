// Prefix a JNI type name with the automerge.repo package path.
macro_rules! repo_classname {
    ($name:literal) => {
        ::jni::jni_str!("org/automerge/repo/", $name)
    };
}

/// Wrapper around `jni::native_method!` that injects the `java_type` and the
/// `type_map` common to every native method on `org.automerge.repo.RepoSys`.
///
/// Mirrors `ams_native!` in `crate::macros` but for the repo layer. See that
/// macro's docs for a narrative explanation of why the wrapper exists and why
/// we refer to bound types by path (`repo::bindings::X`) rather than by bare
/// identifier.
///
/// **To add a new bound repo type, append one line to the `type_map` below.**
macro_rules! repo_native {
    ($($tt:tt)*) => {
        ::jni::native_method! {
            java_type = org.automerge.repo.RepoSys,
            type_map = {
                // ID types
                repo_bindings::DocumentId           => org.automerge.repo.DocumentId,
                repo_bindings::PeerId               => org.automerge.repo.PeerId,
                repo_bindings::StorageId            => org.automerge.repo.StorageId,
                repo_bindings::CommandId            => org.automerge.repo.CommandId,
                repo_bindings::IoTaskId             => org.automerge.repo.IoTaskId,
                repo_bindings::DocumentActorId      => org.automerge.repo.DocumentActorId,
                repo_bindings::AutomergeUrl         => org.automerge.repo.AutomergeUrl,

                // Storage
                repo_bindings::StorageKey           => org.automerge.repo.StorageKey,
                repo_bindings::StorageResult        => org.automerge.repo.StorageResult,
                repo_bindings::StorageTask          => org.automerge.repo.StorageTask,
                repo_bindings::IoTask               => org.automerge.repo.IoTask,
                repo_bindings::IoResult             => org.automerge.repo.IoResult,

                // Loader
                repo_bindings::LoaderStepResult     => org.automerge.repo.LoaderStepResult,

                // Hub results
                repo_bindings::HubResults           => org.automerge.repo.HubResults,
                repo_bindings::HubCommandResult     => org.automerge.repo.HubCommandResult,
                repo_bindings::CommandResult        => org.automerge.repo.CommandResult,
                repo_bindings::ConnectionId         => org.automerge.repo.ConnectionId,
                repo_bindings::DialerId             => org.automerge.repo.DialerId,
                repo_bindings::ListenerId           => org.automerge.repo.ListenerId,
                repo_bindings::DialerConfig         => org.automerge.repo.DialerConfig,
                repo_bindings::ListenerConfig       => org.automerge.repo.ListenerConfig,

                // Document actor + messages + IO
                repo_bindings::DocumentActor        => org.automerge.repo.DocumentActor,
                repo_bindings::DocActorResult       => org.automerge.repo.DocActorResult,
                repo_bindings::SpawnedActor         => org.automerge.repo.SpawnedActor,
                repo_bindings::WithDocResult        => org.automerge.repo.WithDocResult,

                // Common types from `crate::bindings` that the repo layer
                // also needs — kept on this side so the repo macro can
                // resolve their JNI mangled names. Uses the full crate
                // path because `bindings` isn't necessarily imported at
                // every repo_native! call site.
                crate::bindings::Function           => java.util.function.Function,
                repo_bindings::HubToDocMsg          => org.automerge.repo.HubToDocMsg,
                repo_bindings::DocToHubMsg          => org.automerge.repo.DocToHubMsg,
                repo_bindings::ActorMessage         => org.automerge.repo.ActorMessage,
                repo_bindings::DocumentIoTask       => org.automerge.repo.DocumentIoTask,
                repo_bindings::DocumentIoResult     => org.automerge.repo.DocumentIoResult,
                repo_bindings::DocumentChanged      => org.automerge.repo.DocumentChanged,
                repo_bindings::HubIoAction          => org.automerge.repo.HubIoAction,
                repo_bindings::HubIoResult          => org.automerge.repo.HubIoResult,

                // Pointer wrappers (RepoSys inner classes)
                repo_bindings::HubPointer           => org.automerge.repo.RepoSys::HubPointer,
                repo_bindings::DocumentActorPointer => org.automerge.repo.RepoSys::DocumentActorPointer,
                repo_bindings::SamodLoaderPointer   => org.automerge.repo.RepoSys::SamodLoaderPointer,
            },
            $($tt)*
        }
    };
}
