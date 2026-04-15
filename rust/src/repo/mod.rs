//! JNI bindings for the automerge-repo Java API.
//!
//! This module is intentionally separate from the core JNI layer — it binds
//! to its own Java class (`org.automerge.repo.RepoSys`) and keeps its own
//! typed wrappers in [`bindings`]. The long-term intent is that the repo
//! layer could be shipped as a separate Java package; keeping its Rust
//! footprint self-contained makes that straightforward.
//!
//! The public Java entry points live under `org.automerge.repo.*` and are
//! orchestrated by `RepoRuntime.java` (pure Java); the Rust code here is
//! stateless-per-call, wrapping `samod-core` actors and returning structured
//! results.

#[macro_use]
mod macros;

pub(crate) mod bindings;

mod document_actor;
mod hub;
mod ids;
mod loader;
mod pointers;
mod storage;
