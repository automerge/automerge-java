//! Conversions between samod-core's `StorageKey` / `StorageTask` /
//! `StorageResult` types and their `org.automerge.repo` Java counterparts.
//!
//! Used by the loader (`SamodLoader::step` returns IO tasks the Java side
//! must execute, then the results come back via
//! `provideSamodLoaderIoResult`) and later by the document actor IO path.

use jni::{
    objects::{JByteArray, JObject, JObjectArray, JString},
    refs::Reference,
    strings::JNIString,
};
use samod_core::{
    io::{IoResult, IoTask, IoTaskId, StorageResult, StorageTask},
    StorageKey,
};

use crate::{
    bindings::{ArrayList, Map as JMap, MapEntryView},
    interop::throw_illegal_argument,
    repo::bindings as repo_bindings,
};

// StorageKey -----------------------------------------------------------

pub(crate) fn storage_key_to_java<'local>(
    env: &mut jni::Env<'local>,
    key: &StorageKey,
) -> jni::errors::Result<repo_bindings::StorageKey<'local>> {
    let parts: Vec<String> = key.into_iter().cloned().collect();
    let arr = JObjectArray::<JString>::new(env, parts.len(), &JString::null())?;
    for (i, part) in parts.iter().enumerate() {
        let jstr = env.new_string(part)?;
        arr.set_element(env, i, jstr)?;
    }
    repo_bindings::StorageKey::new(env, &arr)
}

pub(crate) fn storage_key_from_java<'local>(
    env: &mut jni::Env<'local>,
    key: &repo_bindings::StorageKey<'local>,
) -> jni::errors::Result<StorageKey> {
    let arr = key.parts(env)?;
    let len = arr.len(env)?;
    let mut parts = Vec::with_capacity(len);
    for i in 0..len {
        let elem = arr.get_element(env, i)?;
        let jstr = JString::cast_local(env, elem)?;
        parts.push(jstr.to_string());
    }
    match StorageKey::from_parts(parts) {
        Ok(k) => Ok(k),
        Err(e) => {
            throw_illegal_argument(env, &JNIString::from(format!("invalid storage key: {}", e)))?;
            Err(jni::errors::Error::JavaException)
        }
    }
}

// StorageTask ---------------------------------------------------------

pub(crate) fn storage_task_to_java<'local>(
    env: &mut jni::Env<'local>,
    task: &StorageTask,
) -> jni::errors::Result<repo_bindings::StorageTask<'local>> {
    let obj: repo_bindings::StorageTask<'local> = match task {
        StorageTask::Load { key } => {
            let jkey = storage_key_to_java(env, key)?;
            repo_bindings::StorageTaskLoad::new(env, &jkey)?.into()
        }
        StorageTask::LoadRange { prefix } => {
            let jprefix = storage_key_to_java(env, prefix)?;
            repo_bindings::StorageTaskLoadRange::new(env, &jprefix)?.into()
        }
        StorageTask::Put { key, value } => {
            let jkey = storage_key_to_java(env, key)?;
            let jbytes = env.byte_array_from_slice(value)?;
            repo_bindings::StorageTaskPut::new(env, &jkey, &jbytes)?.into()
        }
        StorageTask::Delete { key } => {
            let jkey = storage_key_to_java(env, key)?;
            repo_bindings::StorageTaskDelete::new(env, &jkey)?.into()
        }
    };
    Ok(obj)
}

// StorageResult --------------------------------------------------------

pub(crate) fn storage_result_from_java<'local>(
    env: &mut jni::Env<'local>,
    obj: JObject<'local>,
) -> jni::errors::Result<StorageResult> {
    if env.is_instance_of(
        &obj,
        repo_bindings::StorageResultLoad::class_name().as_ref(),
    )? {
        let load = repo_bindings::StorageResultLoad::cast_local(env, obj)?;
        let opt = load.value(env)?;
        let value = optional_byte_array_to_vec(env, opt)?;
        Ok(StorageResult::Load { value })
    } else if env.is_instance_of(
        &obj,
        repo_bindings::StorageResultLoadRange::class_name().as_ref(),
    )? {
        let lr = repo_bindings::StorageResultLoadRange::cast_local(env, obj)?;
        let map = lr.values(env)?;
        let values = map_to_storage_values(env, map.into())?;
        Ok(StorageResult::LoadRange { values })
    } else if env.is_instance_of(&obj, repo_bindings::StorageResultPut::class_name().as_ref())? {
        Ok(StorageResult::Put)
    } else if env.is_instance_of(
        &obj,
        repo_bindings::StorageResultDelete::class_name().as_ref(),
    )? {
        Ok(StorageResult::Delete)
    } else {
        throw_illegal_argument(env, &JNIString::from("unknown StorageResult subtype"))?;
        Err(jni::errors::Error::JavaException)
    }
}

fn optional_byte_array_to_vec<'local>(
    env: &mut jni::Env<'local>,
    opt: JObject<'local>,
) -> jni::errors::Result<Option<Vec<u8>>> {
    // `bindings::Optional` only carries `of`/`empty`/`is_present`/`get`, so
    // cast the incoming JObject and use the typed accessors.
    let opt = crate::bindings::Optional::cast_local(env, opt)?;
    if !opt.is_present(env)? {
        return Ok(None);
    }
    let inner = opt.get(env)?;
    let bytes_array = JByteArray::cast_local(env, inner)?;
    Ok(Some(env.convert_byte_array(&bytes_array)?))
}

fn map_to_storage_values<'local>(
    env: &mut jni::Env<'local>,
    map: JObject<'local>,
) -> jni::errors::Result<std::collections::HashMap<StorageKey, Vec<u8>>> {
    use std::collections::HashMap;

    let map = JMap::cast_local(env, map)?;
    let entry_set = map.entry_set(env)?;
    let iter = entry_set.iterator(env)?;

    let mut out: HashMap<StorageKey, Vec<u8>> = HashMap::new();
    while iter.has_next(env)? {
        let entry_obj = iter.next(env)?;
        let entry = MapEntryView::cast_local(env, entry_obj)?;
        let key_obj = entry.get_key(env)?;
        let value_obj = entry.get_value(env)?;

        let java_key = repo_bindings::StorageKey::cast_local(env, key_obj)?;
        let rust_key = storage_key_from_java(env, &java_key)?;
        let bytes_array = JByteArray::cast_local(env, value_obj)?;
        let rust_value = env.convert_byte_array(&bytes_array)?;
        out.insert(rust_key, rust_value);
    }
    Ok(out)
}

// IoTask<StorageTask> --------------------------------------------------

pub(crate) fn storage_io_task_to_java<'local>(
    env: &mut jni::Env<'local>,
    task: &IoTask<StorageTask>,
) -> jni::errors::Result<repo_bindings::IoTask<'local>> {
    let task_id = io_task_id_to_java(env, task.task_id)?;
    let action = storage_task_to_java(env, &task.action)?;
    let action_obj: JObject = action.into();
    repo_bindings::IoTask::new(env, &task_id, &action_obj)
}

fn io_task_id_to_java<'local>(
    env: &mut jni::Env<'local>,
    id: IoTaskId,
) -> jni::errors::Result<repo_bindings::IoTaskId<'local>> {
    repo_bindings::IoTaskId::new(env, u32::from(id) as i32)
}

fn io_task_id_from_java<'local>(
    env: &mut jni::Env<'local>,
    id: &repo_bindings::IoTaskId<'local>,
) -> jni::errors::Result<IoTaskId> {
    let v = id.value(env)?;
    Ok(IoTaskId::from(v as u32))
}

pub(crate) fn storage_io_result_from_java<'local>(
    env: &mut jni::Env<'local>,
    result: repo_bindings::IoResult<'local>,
) -> jni::errors::Result<IoResult<StorageResult>> {
    let task_id_obj = result.task_id(env)?;
    let task_id = io_task_id_from_java(env, &task_id_obj)?;
    let payload_obj = result.payload(env)?;
    let payload = storage_result_from_java(env, payload_obj)?;
    Ok(IoResult { task_id, payload })
}

// Build a Java ArrayList<IoTask<StorageTask>> from a slice of Rust IoTasks.
pub(crate) fn storage_io_task_list_to_java<'local>(
    env: &mut jni::Env<'local>,
    tasks: &[IoTask<StorageTask>],
) -> jni::errors::Result<ArrayList<'local>> {
    let list = ArrayList::new(env)?;
    for task in tasks {
        let java_task = storage_io_task_to_java(env, task)?;
        let task_obj: JObject = java_task.into();
        list.add(env, &task_obj)?;
    }
    Ok(list)
}
