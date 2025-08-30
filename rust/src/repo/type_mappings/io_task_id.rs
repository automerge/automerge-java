use jni::{
    objects::{JObject, JValue},
    JNIEnv,
};
use samod_core::io::IoTaskId;

pub(crate) const IO_TASK_ID_CLASS: &str = am_classname!("IoTaskId");

pub(crate) fn io_task_id_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    io_task_id: &IoTaskId,
) -> Result<JObject<'local>, jni::errors::Error> {
    // Use the From<IoTaskId> for u32 trait implementation
    let id_value: u32 = (*io_task_id).into();

    // Create IoTaskId object using constructor
    let args = [JValue::from(id_value as i64)]; // Convert u32 to i64 for Java long
    let obj = env.new_object(IO_TASK_ID_CLASS, "(J)V", &args)?;

    Ok(obj)
}
