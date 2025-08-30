use crate::repo::type_mappings::{
    io_task_id::{io_task_id_to_java_object, IO_TASK_ID_CLASS},
    storage_task::storage_task_to_java_object,
};
use jni::{
    objects::{JObject, JValue},
    JNIEnv,
};
use samod_core::io::{IoTask, StorageTask};

pub(crate) const IO_TASK_CLASS: &str = am_classname!("IoTask");

pub fn io_task_storage_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    io_task: &IoTask<StorageTask>,
) -> Result<JObject<'local>, jni::errors::Error> {
    // Convert the task ID
    let task_id_obj = io_task_id_to_java_object(env, &io_task.task_id)?;

    // Convert the storage task action
    let action_obj = storage_task_to_java_object(env, &io_task.action)?;

    // Create IoTask object using constructor (IoTaskId, Object)
    let args = [JValue::from(&task_id_obj), JValue::from(&action_obj)];

    let obj = env.new_object(
        IO_TASK_CLASS,
        format!("(L{};Ljava/lang/Object;)V", IO_TASK_ID_CLASS),
        &args,
    )?;

    Ok(obj)
}
