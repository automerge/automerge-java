//! `JavaPointer` impls for samod-core types stored on the Java side as
//! `RepoSys.*Pointer` wrapper objects.
//!
//! Each impl wires a Rust pointer-bearing type to its corresponding bound
//! Java wrapper from [`super::bindings`]. The shared trait machinery in
//! [`crate::interop::JavaPointer`] handles `alloc_object` /
//! `set_rust_field` / `take_rust_field` / `borrow_from_pointer`.

use jni::strings::JNIStr;
use samod_core::actors::document::DocumentActor;
use samod_core::actors::hub::Hub;
use samod_core::actors::{DocToHubMsg, HubToDocMsg};
use samod_core::SamodLoader;

use crate::interop::JavaPointer;
use crate::repo::bindings as repo_bindings;

impl JavaPointer for SamodLoader {
    type Wrapper<'local> = repo_bindings::SamodLoaderPointer<'local>;
    const POINTER_CLASS: &'static JNIStr = repo_classname!("RepoSys$SamodLoaderPointer");
}

impl JavaPointer for Hub {
    type Wrapper<'local> = repo_bindings::HubPointer<'local>;
    const POINTER_CLASS: &'static JNIStr = repo_classname!("RepoSys$HubPointer");
}

impl JavaPointer for DocumentActor {
    type Wrapper<'local> = repo_bindings::DocumentActorPointer<'local>;
    const POINTER_CLASS: &'static JNIStr = repo_classname!("RepoSys$DocumentActorPointer");
}

impl JavaPointer for HubToDocMsg {
    type Wrapper<'local> = repo_bindings::HubToDocMsg<'local>;
    const POINTER_CLASS: &'static JNIStr = repo_classname!("HubToDocMsg");
}

impl JavaPointer for DocToHubMsg {
    type Wrapper<'local> = repo_bindings::DocToHubMsg<'local>;
    const POINTER_CLASS: &'static JNIStr = repo_classname!("DocToHubMsg");
}
