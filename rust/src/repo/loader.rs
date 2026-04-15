//! Native methods for `RepoSys.createSamodLoader`,
//! `RepoSys.stepSamodLoader`, `RepoSys.provideSamodLoaderIoResult`, and
//! `RepoSys.freeSamodLoader`.
//!
//! The loader drives the storage round-trip required to bring a samod repo
//! up — see `RepoLoader.java` on the Java side for the orchestration.

use jni::{
    objects::{JClass, JList, JObject, JString},
    sys::jlong,
    NativeMethod,
};
use samod_core::{LoaderState, SamodLoader, UnixTimestamp};

use crate::interop::JavaPointer;
use crate::repo::{
    bindings as repo_bindings,
    storage::{storage_io_result_from_java, storage_io_task_list_to_java},
};

const _METHODS: &[NativeMethod] = &[
    repo_native! { static extern fn create_samod_loader(peer_id: JString) -> repo_bindings::SamodLoaderPointer },
    repo_native! { static extern fn step_samod_loader(loader: repo_bindings::SamodLoaderPointer, timestamp: jlong) -> repo_bindings::LoaderStepResult },
    repo_native! { static extern fn provide_samod_loader_io_result(loader: repo_bindings::SamodLoaderPointer, result: repo_bindings::IoResult) },
    repo_native! { static extern fn free_samod_loader(loader: repo_bindings::SamodLoaderPointer) },
];

fn create_samod_loader<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    peer_id: JString<'local>,
) -> jni::errors::Result<repo_bindings::SamodLoaderPointer<'local>> {
    let peer_id_str = peer_id.to_string();
    let peer_id = samod_core::PeerId::from_string(peer_id_str);
    let loader = SamodLoader::new(peer_id);
    unsafe { loader.store_as_pointer(env) }
}

fn step_samod_loader<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    loader: repo_bindings::SamodLoaderPointer<'local>,
    timestamp: jlong,
) -> jni::errors::Result<repo_bindings::LoaderStepResult<'local>> {
    let timestamp = UnixTimestamp::from_millis(timestamp as u128);
    let state = {
        let mut loader_guard = unsafe { SamodLoader::borrow_from_pointer(env, loader)? };
        loader_guard.step(&mut rand::rng(), timestamp)
    };
    loader_state_to_java(env, state)
}

fn provide_samod_loader_io_result<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    loader: repo_bindings::SamodLoaderPointer<'local>,
    result: repo_bindings::IoResult<'local>,
) -> jni::errors::Result<()> {
    let io_result = storage_io_result_from_java(env, result)?;
    let mut loader_guard = unsafe { SamodLoader::borrow_from_pointer(env, loader)? };
    loader_guard.provide_io_result(io_result);
    Ok(())
}

fn free_samod_loader<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    loader: repo_bindings::SamodLoaderPointer<'local>,
) -> jni::errors::Result<()> {
    let _loader = unsafe { SamodLoader::take_from_pointer(env, loader)? };
    Ok(())
}

fn loader_state_to_java<'local>(
    env: &mut jni::Env<'local>,
    state: LoaderState,
) -> jni::errors::Result<repo_bindings::LoaderStepResult<'local>> {
    match state {
        LoaderState::NeedIo(tasks) => {
            let list = storage_io_task_list_to_java(env, &tasks)?;
            // ArrayList implements List, but the Rust type-system doesn't
            // know that — cast through `JObject` to a `JList`, which is
            // what the constructor signature wants.
            let list_obj: JObject = list.into();
            let jlist = JList::cast_local(env, list_obj)?;
            Ok(repo_bindings::LoaderStepResultNeedIo::new(env, &jlist)?.into())
        }
        LoaderState::Loaded(hub) => {
            let hub_ptr = unsafe { (*hub).store_as_pointer(env)? };
            Ok(repo_bindings::LoaderStepResultLoaded::new(env, &hub_ptr)?.into())
        }
    }
}
