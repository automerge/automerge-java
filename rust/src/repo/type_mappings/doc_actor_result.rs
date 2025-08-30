use crate::repo::type_mappings::document_changed::document_changed_to_java_object;
use crate::repo::type_mappings::document_io_task::document_io_task_to_java_object;
use crate::repo::type_mappings::{
    doc_to_hub_msg::doc_to_hub_msg_to_java_object, io_task_id::io_task_id_to_java_object,
};
use jni::{
    objects::{JObject, JValue},
    JNIEnv,
};
use samod_core::actors::document::DocActorResult;

pub(crate) fn doc_actor_result_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    result: &DocActorResult,
) -> Result<JObject<'local>, jni::errors::Error> {
    // Convert io_tasks: Vec<IoTask<DocumentIoTask>> to Java array
    let io_tasks_array = {
        let task_objects: Result<Vec<_>, _> = result
            .io_tasks
            .iter()
            .map(|task| {
                // Convert IoTask<DocumentIoTask> to Java IoTask<DocumentIoTask>
                let task_id_obj = io_task_id_to_java_object(env, &task.task_id)?;
                let action_obj = document_io_task_to_java_object(env, &task.action)?;

                let io_task_obj = env.new_object(
                    am_classname!("IoTask"),
                    format!("(L{};Ljava/lang/Object;)V", am_classname!("IoTaskId")),
                    &[JValue::from(&task_id_obj), JValue::from(&action_obj)],
                )?;
                Ok::<JObject, jni::errors::Error>(io_task_obj)
            })
            .collect();

        let task_objects = task_objects?;
        let io_task_class = env.find_class(am_classname!("IoTask"))?;
        let array =
            env.new_object_array(task_objects.len() as i32, io_task_class, JObject::null())?;

        for (i, task_obj) in task_objects.iter().enumerate() {
            env.set_object_array_element(&array, i as i32, task_obj)?;
        }
        array
    };

    // Convert outgoing_messages: Vec<DocToHubMsg> to Java array
    let outgoing_messages_array = {
        let message_objects: Result<Vec<_>, _> = result
            .outgoing_messages
            .iter()
            .map(|msg| doc_to_hub_msg_to_java_object(env, msg))
            .collect();

        let message_objects = message_objects?;
        let doc_to_hub_msg_class = env.find_class(am_classname!("DocToHubMsg"))?;
        let array = env.new_object_array(
            message_objects.len() as i32,
            doc_to_hub_msg_class,
            JObject::null(),
        )?;

        for (i, msg_obj) in message_objects.iter().enumerate() {
            env.set_object_array_element(&array, i as i32, msg_obj)?;
        }
        array
    };

    // Convert ephemeral_messages: Vec<Vec<u8>> to Java byte[][]
    let ephemeral_messages_array = {
        let byte_array_class = env.find_class("[B")?;
        let array = env.new_object_array(
            result.ephemeral_messages.len() as i32,
            byte_array_class,
            JObject::null(),
        )?;

        for (i, msg_bytes) in result.ephemeral_messages.iter().enumerate() {
            let byte_array = env.byte_array_from_slice(msg_bytes)?;
            env.set_object_array_element(&array, i as i32, &JObject::from(byte_array))?;
        }
        array
    };

    // Convert change_events: Vec<DocumentChanged> to Java array
    let change_events_array = {
        let event_objects: Result<Vec<_>, _> = result
            .change_events
            .iter()
            .map(|event| document_changed_to_java_object(env, event))
            .collect();

        let event_objects = event_objects?;
        let document_changed_class = env.find_class(am_classname!("DocumentChanged"))?;
        let array = env.new_object_array(
            event_objects.len() as i32,
            document_changed_class,
            JObject::null(),
        )?;

        for (i, event_obj) in event_objects.iter().enumerate() {
            env.set_object_array_element(&array, i as i32, event_obj)?;
        }
        array
    };

    env.new_object(
        am_classname!("DocActorResult"),
        format!(
            "([L{};[L{};[[B[L{};Z)V",
            am_classname!("IoTask"),
            am_classname!("DocToHubMsg"),
            am_classname!("DocumentChanged")
        ),
        &[
            JValue::from(&io_tasks_array),
            JValue::from(&outgoing_messages_array),
            JValue::from(&ephemeral_messages_array),
            JValue::from(&change_events_array),
            JValue::Bool(result.stopped as jni::sys::jboolean),
        ],
    )
}
