use jni::{
    objects::{JObject, JValue},
    JNIEnv,
};
use samod_core::ConnectionId;

pub(crate) const CONNECTION_ID_CLASS: &str = am_classname!("ConnectionId");

/// Convert a Rust ConnectionId (created by Hub actor) to a Java ConnectionId object
pub(crate) fn connection_id_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    connection_id: &ConnectionId,
) -> Result<JObject<'local>, jni::errors::Error> {
    // Use the From<ConnectionId> for u32 trait implementation
    let id_value: u32 = (*connection_id).into();

    // Create ConnectionId object using constructor
    let args = [JValue::from(id_value as i32)]; // Convert u32 to i32 for Java
    let obj = env.new_object(CONNECTION_ID_CLASS, "(I)V", &args)?;

    Ok(obj)
}

/// Convert a Java ConnectionId object to a Rust ConnectionId
pub(crate) fn java_object_to_connection_id<'local>(
    env: &mut JNIEnv<'local>,
    obj: JObject<'local>,
) -> Result<ConnectionId, jni::errors::Error> {
    // Get the id field from the Java object
    let id_field = env.get_field(&obj, "id", "I")?;
    let id_value = id_field.i()? as u32;

    // Convert u32 to ConnectionId using From trait
    Ok(ConnectionId::from(id_value))
}
