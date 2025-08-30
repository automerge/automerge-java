use jni::{
    objects::{JObject, JValue},
    JNIEnv,
};
use samod_core::CommandId;

pub(crate) const COMMAND_ID_CLASS: &str = am_classname!("CommandId");

pub(crate) fn command_id_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    command_id: &CommandId,
) -> Result<JObject<'local>, jni::errors::Error> {
    // Use the From<CommandId> for u32 trait implementation
    let id_value: u32 = (*command_id).into();

    // Create CommandId object using constructor
    let args = [JValue::from(id_value as i64)]; // Convert u32 to i64 for Java long
    let obj = env.new_object(COMMAND_ID_CLASS, "(J)V", &args)?;

    Ok(obj)
}
