use jni::{
    objects::{JObject, JValue},
    JNIEnv,
};
use samod_core::actors::hub::DispatchedCommand;

use crate::{interop::AsPointerObj, repo::type_mappings::command_id::command_id_to_java_object};

/// Convert a Rust DispatchedCommand to a Java DispatchedCommand object
pub(crate) fn dispatched_command_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    dispatched_command: DispatchedCommand,
) -> Result<JObject<'local>, jni::errors::Error> {
    // Convert CommandId to Java object
    let command_id_java = command_id_to_java_object(env, &dispatched_command.command_id)?;

    // Convert HubEvent to Java object using pointer
    let hub_event_ptr = match dispatched_command.event.to_pointer_obj(env) {
        Ok(ptr) => ptr,
        Err(_) => return Err(jni::errors::Error::JavaException),
    };
    let hub_event_java = env.new_object(
        am_classname!("HubEvent"),
        "(Lorg/automerge/AutomergeSys$HubEventPointer;)V",
        &[(&hub_event_ptr).into()],
    )?;

    // Create DispatchedCommand using package-private constructor
    let dispatched_command_java = env.new_object(
        am_classname!("DispatchedCommand"),
        "(Lorg/automerge/CommandId;Lorg/automerge/HubEvent;)V",
        &[
            JValue::Object(&command_id_java),
            JValue::Object(&hub_event_java),
        ],
    )?;

    Ok(dispatched_command_java)
}
