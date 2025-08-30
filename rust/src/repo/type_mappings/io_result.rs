use crate::repo::type_mappings::document_io_result::java_object_to_document_io_result;
use crate::repo::type_mappings::hub_io_result::java_object_to_hub_io_result;
use crate::repo::type_mappings::io_task_id::IO_TASK_ID_CLASS;
use crate::repo::type_mappings::storage_result::java_object_to_storage_result;
use jni::{objects::JObject, JNIEnv};
use samod_core::actors::document::io::DocumentIoResult;
use samod_core::actors::hub::io::HubIoResult;
use samod_core::io::{IoResult, IoTaskId, StorageResult};

/// Convert a Java IoResult<StorageResult> object to a Rust IoResult<StorageResult>
pub(crate) fn java_object_to_io_result_storage<'local>(
    env: &mut JNIEnv<'local>,
    obj: JObject<'local>,
) -> Result<IoResult<StorageResult>, jni::errors::Error> {
    java_io_result_to_io_result(env, obj, java_object_to_storage_result)
}

/// Convert a Java IoResult<HubIoResult> object to a Rust IoResult<HubIoResult>
pub(crate) fn java_object_to_io_result_hub<'local>(
    env: &mut JNIEnv<'local>,
    obj: JObject<'local>,
) -> Result<IoResult<HubIoResult>, jni::errors::Error> {
    java_io_result_to_io_result(env, obj, java_object_to_hub_io_result)
}

pub(crate) fn java_object_to_io_result_document<'local>(
    env: &mut JNIEnv<'local>,
    obj: JObject<'local>,
) -> Result<IoResult<DocumentIoResult>, jni::errors::Error> {
    java_io_result_to_io_result(env, obj, java_object_to_document_io_result)
}

pub(crate) fn java_io_result_to_io_result<'local, F, T>(
    env: &mut JNIEnv<'local>,
    obj: JObject<'local>,
    parse_payload: F,
) -> Result<IoResult<T>, jni::errors::Error>
where
    F: FnOnce(&mut JNIEnv<'local>, JObject<'local>) -> Result<T, jni::errors::Error>,
{
    // Get the taskId field
    let task_id_field = env.get_field(&obj, "taskId", format!("L{};", IO_TASK_ID_CLASS))?;
    let task_id_obj = task_id_field.l()?;

    // Extract the long value from the IoTaskId object
    let task_id_value = env.get_field(&task_id_obj, "id", "J")?;
    let task_id_long = task_id_value.j()? as usize;
    let task_id = IoTaskId::from(task_id_long);

    // Get the payload field
    let payload_field = env.get_field(&obj, "payload", "Ljava/lang/Object;")?;
    let payload_obj = payload_field.l()?;

    // Convert the payload to HubIoResult
    let payload = parse_payload(env, payload_obj)?;

    Ok(IoResult { task_id, payload })
}
