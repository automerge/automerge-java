use crate::repo::type_mappings::connection_id::connection_id_to_java_object;
use crate::repo::type_mappings::document_actor_id::document_actor_id_to_java_object;
use crate::repo::type_mappings::document_id::document_id_to_java_object;
use jni::{objects::JObject, JNIEnv};
use samod_core::CommandResult;

/// Convert a Rust CommandResult to a Java CommandResult object
pub(crate) fn command_result_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    command_result: &CommandResult,
) -> Result<JObject<'local>, jni::errors::Error> {
    match command_result {
        CommandResult::CreateConnection { connection_id } => {
            // Convert ConnectionId to Java
            let connection_id_java = connection_id_to_java_object(env, connection_id)?;

            // Create CommandResult.CreateConnection object
            let create_connection_obj = env.new_object(
                am_classname!("CommandResult$CreateConnection"),
                "(Lorg/automerge/ConnectionId;)V",
                &[jni::objects::JValue::Object(&connection_id_java)],
            )?;

            Ok(create_connection_obj)
        }

        CommandResult::DisconnectConnection => {
            // Create CommandResult.DisconnectConnection object (no parameters)
            let disconnect_obj = env.new_object(
                am_classname!("CommandResult$DisconnectConnection"),
                "()V",
                &[],
            )?;

            Ok(disconnect_obj)
        }

        CommandResult::Receive {
            connection_id,
            error,
        } => {
            // Convert ConnectionId to Java
            let connection_id_java = connection_id_to_java_object(env, connection_id)?;

            // Convert error Option<String> to Java String (nullable)
            let error_java = match error {
                Some(err_msg) => env.new_string(err_msg)?.into(),
                None => JObject::null(),
            };

            // Create CommandResult.Receive object
            let receive_obj = env.new_object(
                am_classname!("CommandResult$Receive"),
                "(Lorg/automerge/ConnectionId;Ljava/lang/String;)V",
                &[
                    jni::objects::JValue::Object(&connection_id_java),
                    jni::objects::JValue::Object(&error_java),
                ],
            )?;

            Ok(receive_obj)
        }

        CommandResult::ActorReady => {
            // Create CommandResult.ActorReady object (no parameters)
            let actor_ready_obj =
                env.new_object(am_classname!("CommandResult$ActorReady"), "()V", &[])?;

            Ok(actor_ready_obj)
        }

        CommandResult::CreateDocument {
            actor_id,
            document_id,
        } => {
            // Convert DocumentActorId to Java
            let actor_id_java = document_actor_id_to_java_object(env, actor_id)?;

            // Convert DocumentId to Java
            let document_id_java = document_id_to_java_object(env, document_id)?;

            // Create CommandResult.CreateDocument object
            let create_document_obj = env.new_object(
                am_classname!("CommandResult$CreateDocument"),
                "(Lorg/automerge/DocumentActorId;Lorg/automerge/DocumentId;)V",
                &[
                    jni::objects::JValue::Object(&actor_id_java),
                    jni::objects::JValue::Object(&document_id_java),
                ],
            )?;

            Ok(create_document_obj)
        }

        CommandResult::FindDocument { actor_id, found } => {
            // Convert DocumentActorId to Java
            let actor_id_java = document_actor_id_to_java_object(env, actor_id)?;

            // Convert bool to Java boolean
            let found_java = jni::objects::JValue::Bool(*found as jni::sys::jboolean);

            // Create CommandResult.FindDocument object
            let find_document_obj = env.new_object(
                am_classname!("CommandResult$FindDocument"),
                "(Lorg/automerge/DocumentActorId;Z)V",
                &[jni::objects::JValue::Object(&actor_id_java), found_java],
            )?;

            Ok(find_document_obj)
        }
    }
}
