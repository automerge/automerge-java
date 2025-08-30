use jni::{
    objects::{JByteArray, JObject},
    JNIEnv,
};
use samod_core::actors::hub::io::HubIoAction;

use crate::repo::type_mappings::connection_id::{
    connection_id_to_java_object, java_object_to_connection_id, CONNECTION_ID_CLASS,
};

pub(crate) const HUB_IO_ACTION_SEND_CLASS: &str = am_classname!("HubIoAction$Send");
pub(crate) const HUB_IO_ACTION_DISCONNECT_CLASS: &str = am_classname!("HubIoAction$Disconnect");

pub(crate) fn hub_io_action_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    action: &HubIoAction,
) -> Result<JObject<'local>, jni::errors::Error> {
    match action {
        HubIoAction::Send { connection_id, msg } => {
            // Convert ConnectionId to Java
            let connection_id_java = connection_id_to_java_object(env, connection_id)?;

            // Convert Vec<u8> to Java byte array
            let msg_array = env.byte_array_from_slice(msg)?;

            // Create HubIoAction.Send object
            let send_obj = env.new_object(
                HUB_IO_ACTION_SEND_CLASS,
                format!("(L{};[B)V", CONNECTION_ID_CLASS),
                &[
                    jni::objects::JValue::Object(&connection_id_java),
                    jni::objects::JValue::Object(&JObject::from(msg_array)),
                ],
            )?;

            Ok(send_obj)
        }

        HubIoAction::Disconnect { connection_id } => {
            // Convert ConnectionId to Java
            let connection_id_java = connection_id_to_java_object(env, connection_id)?;

            // Create HubIoAction.Disconnect object
            let disconnect_obj = env.new_object(
                HUB_IO_ACTION_DISCONNECT_CLASS,
                format!("(L{};)V", CONNECTION_ID_CLASS),
                &[jni::objects::JValue::Object(&connection_id_java)],
            )?;

            Ok(disconnect_obj)
        }
    }
}

/// Convert a Java HubIoAction object to a Rust HubIoAction
pub unsafe fn java_object_to_hub_io_action(
    env: &mut JNIEnv,
    obj: JObject,
) -> Result<HubIoAction, jni::errors::Error> {
    // Get the class name to determine which variant this is
    let class = env.get_object_class(&obj)?;
    let class_name = env.call_method(class, "getName", "()Ljava/lang/String;", &[])?;
    let class_name_str: String = env.get_string(&class_name.l()?.into())?.into();
    let nested_class_name = class_name_str.split("$").nth(1).unwrap();

    match nested_class_name {
        "Send" => {
            // Extract ConnectionId field
            let connection_id_field =
                env.get_field(&obj, "connectionId", "Lorg/automerge/ConnectionId;")?;
            let connection_id_obj = connection_id_field.l()?;
            let connection_id = java_object_to_connection_id(env, connection_id_obj)?;

            // Extract message field
            let message_field = env.get_field(&obj, "message", "[B")?;
            let message_array = message_field.l()?.into_raw();
            let message_array = JByteArray::from_raw(message_array);
            let message_vec = env.convert_byte_array(message_array)?;

            Ok(HubIoAction::Send {
                connection_id,
                msg: message_vec,
            })
        }

        "Disconnect" => {
            // Extract ConnectionId field
            let connection_id_field =
                env.get_field(&obj, "connectionId", "Lorg/automerge/ConnectionId;")?;
            let connection_id_obj = connection_id_field.l()?;
            let connection_id = java_object_to_connection_id(env, connection_id_obj)?;

            Ok(HubIoAction::Disconnect { connection_id })
        }

        _ => Err(jni::errors::Error::JavaException),
    }
}
