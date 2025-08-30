use jni::{
    objects::{JObject, JValue},
    JNIEnv,
};
use samod_core::DocumentActorId;

const DOCUMENT_ACTOR_ID_CLASS: &str = am_classname!("DocumentActorId");

/// Convert a Rust DocumentActorId (created by Hub actor) to a Java DocumentActorId object
pub(crate) fn document_actor_id_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    document_actor_id: &DocumentActorId,
) -> Result<JObject<'local>, jni::errors::Error> {
    // Use the From<DocumentActorId> for u32 trait implementation
    let id_value: u32 = (*document_actor_id).into();

    // Create DocumentActorId object using constructor
    let args = [JValue::from(id_value as i32)]; // Convert u32 to i32 for Java
    let obj = env.new_object(DOCUMENT_ACTOR_ID_CLASS, "(I)V", &args)?;

    Ok(obj)
}

/// Convert a Java DocumentActorId object to a Rust DocumentActorId
pub(crate) fn java_object_to_document_actor_id<'local>(
    env: &mut JNIEnv<'local>,
    obj: JObject<'local>,
) -> Result<DocumentActorId, jni::errors::Error> {
    // Get the id field from the Java object
    let id_field = env.get_field(&obj, "id", "I")?;
    let id_value = id_field.i()? as u32;

    // Convert u32 to DocumentActorId using From trait
    Ok(DocumentActorId::from(id_value))
}
