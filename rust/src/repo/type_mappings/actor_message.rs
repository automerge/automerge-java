use crate::repo::type_mappings::{
    document_actor_id::{document_actor_id_to_java_object, java_object_to_document_actor_id},
    hub_to_doc_msg::{hub_to_doc_msg_to_java_object, java_object_to_hub_to_doc_msg},
};
use jni::{objects::JObject, JNIEnv};
use samod_core::{actors::HubToDocMsg, DocumentActorId};

/// Convert a Rust tuple (DocumentActorId, HubToDocMsg) to a Java ActorMessage object
pub(crate) fn actor_message_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    actor_id: &DocumentActorId,
    message: &HubToDocMsg,
) -> Result<JObject<'local>, jni::errors::Error> {
    // Convert DocumentActorId to Java
    let actor_id_java = document_actor_id_to_java_object(env, actor_id).unwrap();

    // Convert HubToDocMsg to Java
    let message_java = hub_to_doc_msg_to_java_object(env, message).unwrap();

    // Create ActorMessage object
    env.new_object(
        am_classname!("ActorMessage"),
        "(Lorg/automerge/DocumentActorId;Lorg/automerge/HubToDocMsg;)V",
        &[
            jni::objects::JValue::Object(&actor_id_java),
            (&message_java).into(),
        ],
    )
}

/// Convert a Java ActorMessage object to a Rust tuple (DocumentActorId, HubToDocMsg)
pub(crate) fn java_object_to_actor_message<'local>(
    env: &mut JNIEnv<'local>,
    obj: JObject<'local>,
) -> Result<(DocumentActorId, HubToDocMsg), jni::errors::Error> {
    // Extract actorId field
    let actor_id_field = env.get_field(&obj, "actorId", "Lorg/automerge/DocumentActorId;")?;
    let actor_id_obj = actor_id_field.l()?;
    let actor_id = java_object_to_document_actor_id(env, actor_id_obj)?;

    // Extract message field
    let message_field = env.get_field(&obj, "message", "Lorg/automerge/HubToDocMsg;")?;
    let message_obj = message_field.l()?;
    let message = java_object_to_hub_to_doc_msg(env, message_obj)?;

    Ok((actor_id, message))
}
