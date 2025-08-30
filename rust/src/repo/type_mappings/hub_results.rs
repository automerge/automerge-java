use crate::repo::type_mappings::command_result::command_result_to_java_object;
use crate::repo::type_mappings::connection_event::connection_event_to_java_object;
use crate::repo::type_mappings::hub_io_action::hub_io_action_to_java_object;
use crate::repo::type_mappings::io_task_id::io_task_id_to_java_object;
use crate::repo::type_mappings::spawn_args::spawn_args_to_java_object;
use crate::repo::type_mappings::{
    actor_message::actor_message_to_java_object, command_id::command_id_to_java_object,
};
use jni::{
    objects::{JObject, JValue},
    JNIEnv,
};
use samod_core::actors::hub::HubResults;

/// Convert a Rust HubResults to a Java HubResults object
pub(crate) fn hub_results_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    results: &HubResults,
) -> Result<JObject<'local>, jni::errors::Error> {
    // Convert new_tasks: Vec<IoTask<HubIoAction>> to Java array
    let new_tasks_array = {
        let task_objects: Vec<_> = results
            .new_tasks
            .iter()
            .map(|task| {
                // Convert IoTask<HubIoAction> to Java IoTask<HubIoAction>
                let task_id_obj = io_task_id_to_java_object(env, &task.task_id)?;
                let action_obj = hub_io_action_to_java_object(env, &task.action)?;

                env.new_object(
                    am_classname!("IoTask"),
                    "(Lorg/automerge/IoTaskId;Ljava/lang/Object;)V",
                    &[JValue::from(&task_id_obj), JValue::from(&action_obj)],
                )
            })
            .collect::<Result<_, _>>()?;

        let io_task_class = env.find_class(am_classname!("IoTask"))?;
        let array =
            env.new_object_array(task_objects.len() as i32, io_task_class, JObject::null())?;

        for (i, task_obj) in task_objects.iter().enumerate() {
            env.set_object_array_element(&array, i as i32, task_obj)?;
        }
        array
    };

    // Convert completed_commands: HashMap<CommandId, CommandResult> to Java Map
    let completed_commands_map = {
        let hashmap_class = env.find_class("java/util/HashMap")?;
        let map = env.new_object(hashmap_class, "()V", &[])?;

        for (command_id, command_result) in &results.completed_commands {
            let command_id_obj = command_id_to_java_object(env, command_id)?;
            let command_result_obj = command_result_to_java_object(env, command_result)?;

            env.call_method(
                &map,
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                &[
                    JValue::from(&command_id_obj),
                    JValue::from(&command_result_obj),
                ],
            )?;
        }
        map
    };

    // Convert spawn_actors: Vec<SpawnArgs> to Java array
    let spawn_actors_array = {
        let spawn_objects: Result<Vec<_>, _> = results
            .spawn_actors
            .iter()
            .map(|spawn_args| spawn_args_to_java_object(env, spawn_args))
            .collect();

        let spawn_objects = spawn_objects?;
        let spawn_args_class = env.find_class(am_classname!("SpawnArgs"))?;
        let array = env.new_object_array(
            spawn_objects.len() as i32,
            spawn_args_class,
            JObject::null(),
        )?;

        for (i, spawn_obj) in spawn_objects.iter().enumerate() {
            env.set_object_array_element(&array, i as i32, &spawn_obj)?;
        }
        array
    };

    // Convert actor_messages: Vec<(DocumentActorId, HubToDocMsg)> to Java array
    let actor_messages_array = {
        let message_objects: Vec<_> = results
            .actor_messages
            .iter()
            .map(|(actor_id, message)| actor_message_to_java_object(env, actor_id, message))
            .collect::<Result<Vec<_>, _>>()?;

        let actor_message_class = env.find_class(am_classname!("ActorMessage"))?;
        let array = env.new_object_array(
            message_objects.len() as i32,
            actor_message_class,
            JObject::null(),
        )?;

        for (i, message_obj) in message_objects.iter().enumerate() {
            env.set_object_array_element(&array, i as i32, message_obj)?;
        }
        array
    };

    // Convert connection_events: Vec<ConnectionEvent> to Java array
    let connection_events_array = {
        let event_objects: Result<Vec<_>, _> = results
            .connection_events
            .iter()
            .map(|event| connection_event_to_java_object(env, event))
            .collect();

        let event_objects = event_objects?;
        let connection_event_class = env.find_class(am_classname!("ConnectionEvent"))?;
        let array = env.new_object_array(
            event_objects.len() as i32,
            connection_event_class,
            JObject::null(),
        )?;

        for (i, event_obj) in event_objects.iter().enumerate() {
            env.set_object_array_element(&array, i as i32, event_obj)?;
        }
        array
    };

    // Create HubResults object
    let hub_results_obj = env.new_object(
        am_classname!("HubResults"),
        "([Lorg/automerge/IoTask;Ljava/util/Map;[Lorg/automerge/SpawnArgs;[Lorg/automerge/ActorMessage;[Lorg/automerge/ConnectionEvent;Z)V",
        &[
            JValue::from(&new_tasks_array),
            JValue::from(&completed_commands_map),
            JValue::from(&spawn_actors_array),
            JValue::from(&actor_messages_array),
            JValue::from(&connection_events_array),
            JValue::Bool(results.stopped as jni::sys::jboolean),
        ],
    )?;

    Ok(hub_results_obj)
}
