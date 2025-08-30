use automerge_jni_macros::jni_fn;
use jni::{
    objects::{JClass, JObject},
    sys::{jboolean, jint, jobject},
    JNIEnv,
};
use samod_core::actors::hub::HubEvent;

use crate::{
    interop::AsPointerObj,
    repo::type_mappings::{
        conn_direction::java_object_to_conn_direction, connection_id::java_object_to_connection_id,
        dispatched_command::dispatched_command_to_java_object,
        doc_to_hub_msg::java_object_to_doc_to_hub_msg,
        document_actor_id::java_object_to_document_actor_id,
        document_id::java_object_to_document_id, io_result::java_object_to_io_result_hub,
    },
};

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn freeHubEvent<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass,
    pointer: jobject,
) {
    let _hub_event = HubEvent::owned_from_pointer_obj(&mut env, pointer);
    // The HubEvent will be dropped here, freeing its memory
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn hubEventEquals<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass,
    pointer1: jobject,
    pointer2: jobject,
) -> jboolean {
    let hub_event1 = match HubEvent::from_pointer_obj(&mut env, pointer1) {
        Ok(event) => event,
        Err(_) => return false as jboolean,
    };

    let hub_event2 = match HubEvent::from_pointer_obj(&mut env, pointer2) {
        Ok(event) => event,
        Err(_) => return false as jboolean,
    };

    // For now, we'll use pointer equality since HubEvent doesn't implement PartialEq
    // In a real implementation, you might want to implement semantic equality
    std::ptr::eq(hub_event1, hub_event2) as jboolean
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn hubEventHashCode<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass,
    pointer: jobject,
) -> jint {
    let hub_event = match HubEvent::from_pointer_obj(&mut env, pointer) {
        Ok(event) => event,
        Err(_) => return 0,
    };

    // Use the memory address as a hash code
    (hub_event as *const HubEvent as usize) as jint
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn hubEventToString(
    mut env: JNIEnv,
    _class: JClass,
    pointer: jobject,
) -> jobject {
    let hub_event = match HubEvent::from_pointer_obj(&mut env, pointer) {
        Ok(event) => event,
        Err(_) => {
            return env
                .new_string("HubEvent{invalid_pointer}")
                .unwrap()
                .into_raw();
        }
    };

    // Use the Debug implementation
    let debug_string = format!("{:?}", hub_event);
    env.new_string(debug_string).unwrap().into_raw()
}

// Simple HubEvent constructors - these will be implemented incrementally

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn createHubEventTick(mut env: JNIEnv, _class: JClass) -> jobject {
    let hub_event = HubEvent::tick();

    match hub_event.to_pointer_obj(&mut env) {
        Ok(ptr_obj) => {
            let java_hub_event = env
                .new_object(
                    am_classname!("HubEvent"),
                    "(Lorg/automerge/AutomergeSys$HubEventPointer;)V",
                    &[(&ptr_obj).into()],
                )
                .unwrap();
            java_hub_event.into_raw()
        }
        Err(e) => {
            env.throw_new(
                "java/lang/IllegalArgumentException",
                format!("Failed to create HubEvent pointer: {}", e),
            )
            .unwrap();
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn createHubEventStop(mut env: JNIEnv, _class: JClass) -> jobject {
    let hub_event = HubEvent::stop();

    match hub_event.to_pointer_obj(&mut env) {
        Ok(ptr_obj) => {
            let java_hub_event = env
                .new_object(
                    am_classname!("HubEvent"),
                    "(Lorg/automerge/AutomergeSys$HubEventPointer;)V",
                    &[(&ptr_obj).into()],
                )
                .unwrap();
            java_hub_event.into_raw()
        }
        Err(e) => {
            env.throw_new(
                "java/lang/IllegalArgumentException",
                format!("Failed to create HubEvent pointer: {}", e),
            )
            .unwrap();
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn createHubEventActorMessage(
    mut env: JNIEnv,
    _class: JClass,
    actor_id: jobject,
    message: jobject,
) -> jobject {
    let actor_id = JObject::from_raw(actor_id);
    let doc_actor_id = match java_object_to_document_actor_id(&mut env, actor_id) {
        Ok(id) => id,
        Err(e) => {
            env.throw_new(
                "java/lang/IllegalArgumentException",
                format!("Failed to convert DocumentActorId: {}", e),
            )
            .unwrap();
            return JObject::null().into_raw();
        }
    };

    let doc_to_hub_msg = match java_object_to_doc_to_hub_msg(&mut env, JObject::from_raw(message)) {
        Ok(msg) => msg,
        Err(e) => {
            env.throw_new(
                "java/lang/IllegalArgumentException",
                format!("Failed to convert DocToHubMsg: {}", e),
            )
            .unwrap();
            return JObject::null().into_raw();
        }
    };

    let hub_event = HubEvent::actor_message(doc_actor_id, doc_to_hub_msg);

    match hub_event.to_pointer_obj(&mut env) {
        Ok(ptr_obj) => {
            let java_hub_event = env
                .new_object(
                    am_classname!("HubEvent"),
                    "(Lorg/automerge/AutomergeSys$HubEventPointer;)V",
                    &[(&ptr_obj).into()],
                )
                .unwrap();
            java_hub_event.into_raw()
        }
        Err(e) => {
            env.throw_new(
                "java/lang/IllegalArgumentException",
                format!("Failed to create HubEvent pointer: {}", e),
            )
            .unwrap();
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn createHubEventReceive(
    mut env: JNIEnv,
    _class: JClass,
    connection_id: jobject,
    message: jni::sys::jbyteArray,
) -> jobject {
    // Convert ConnectionId from Java
    let conn_id = match java_object_to_connection_id(&mut env, JObject::from_raw(connection_id)) {
        Ok(id) => id,
        Err(e) => {
            env.throw_new(
                "java/lang/IllegalArgumentException",
                format!("Failed to convert ConnectionId: {}", e),
            )
            .unwrap();
            return JObject::null().into_raw();
        }
    };

    // Convert byte array to Vec<u8>
    let message_bytes =
        match env.convert_byte_array(jni::objects::JPrimitiveArray::from_raw(message)) {
            Ok(bytes) => bytes,
            Err(e) => {
                env.throw_new(
                    "java/lang/IllegalArgumentException",
                    format!("Failed to convert message bytes: {}", e),
                )
                .unwrap();
                return JObject::null().into_raw();
            }
        };

    // Create the DispatchedCommand using HubEvent::receive
    let dispatched_command = HubEvent::receive(conn_id, message_bytes);

    // Convert to Java object
    match dispatched_command_to_java_object(&mut env, dispatched_command) {
        Ok(java_dispatched_command) => java_dispatched_command.into_raw(),
        Err(e) => {
            env.throw_new(
                "java/lang/IllegalArgumentException",
                format!("Failed to create DispatchedCommand: {}", e),
            )
            .unwrap();
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn createHubEventCreateConnection(
    mut env: JNIEnv,
    _class: JClass,
    direction: jobject,
) -> jobject {
    // Convert ConnDirection from Java
    let conn_direction = match java_object_to_conn_direction(&mut env, JObject::from_raw(direction))
    {
        Ok(dir) => dir,
        Err(e) => {
            env.throw_new(
                "java/lang/IllegalArgumentException",
                format!("Failed to convert ConnDirection: {}", e),
            )
            .unwrap();
            return JObject::null().into_raw();
        }
    };

    // Create the DispatchedCommand using HubEvent::create_connection
    let dispatched_command = HubEvent::create_connection(conn_direction);

    // Convert to Java object
    match dispatched_command_to_java_object(&mut env, dispatched_command) {
        Ok(java_dispatched_command) => java_dispatched_command.into_raw(),
        Err(e) => {
            env.throw_new(
                "java/lang/IllegalArgumentException",
                format!("Failed to create DispatchedCommand: {}", e),
            )
            .unwrap();
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn createHubEventIoComplete(
    mut env: JNIEnv,
    _class: JClass,
    io_result: jobject,
) -> jobject {
    // Convert IoResult from Java
    let io_result_rust =
        java_object_to_io_result_hub(&mut env, JObject::from_raw(io_result)).unwrap();

    // Create the HubEvent using HubEvent::io_complete
    let hub_event = HubEvent::io_complete(io_result_rust);

    match hub_event.to_pointer_obj(&mut env) {
        Ok(ptr_obj) => {
            let java_hub_event = env
                .new_object(
                    am_classname!("HubEvent"),
                    "(Lorg/automerge/AutomergeSys$HubEventPointer;)V",
                    &[(&ptr_obj).into()],
                )
                .unwrap();
            java_hub_event.into_raw()
        }
        Err(e) => {
            env.throw_new(
                "java/lang/IllegalArgumentException",
                format!("Failed to create HubEvent pointer: {}", e),
            )
            .unwrap();
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn createHubEventConnectionLost(
    mut env: JNIEnv,
    _class: JClass,
    connection_id: jobject,
) -> jobject {
    // Convert ConnectionId from Java
    let conn_id = match java_object_to_connection_id(&mut env, JObject::from_raw(connection_id)) {
        Ok(id) => id,
        Err(e) => {
            env.throw_new(
                "java/lang/IllegalArgumentException",
                format!("Failed to convert ConnectionId: {}", e),
            )
            .unwrap();
            return JObject::null().into_raw();
        }
    };

    // Create the HubEvent using HubEvent::connection_lost
    let hub_event = HubEvent::connection_lost(conn_id);

    match hub_event.to_pointer_obj(&mut env) {
        Ok(ptr_obj) => {
            let java_hub_event = env
                .new_object(
                    am_classname!("HubEvent"),
                    "(Lorg/automerge/AutomergeSys$HubEventPointer;)V",
                    &[(&ptr_obj).into()],
                )
                .unwrap();
            java_hub_event.into_raw()
        }
        Err(e) => {
            env.throw_new(
                "java/lang/IllegalArgumentException",
                format!("Failed to create HubEvent pointer: {}", e),
            )
            .unwrap();
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn createHubEventActorReady(
    mut env: JNIEnv,
    _class: JClass,
    document_id: jobject,
) -> jobject {
    let doc_id_obj = JObject::from_raw(document_id);
    // Convert DocumentId from Java
    let doc_id = match java_object_to_document_id(&mut env, doc_id_obj) {
        Ok(id) => id,
        Err(e) => {
            env.throw_new(
                "java/lang/IllegalArgumentException",
                format!("Failed to convert DocumentId: {}", e),
            )
            .unwrap();
            return JObject::null().into_raw();
        }
    };

    // Create the DispatchedCommand using HubEvent::actor_ready
    let dispatched_command = HubEvent::actor_ready(doc_id);

    // Convert to Java object
    match dispatched_command_to_java_object(&mut env, dispatched_command) {
        Ok(java_dispatched_command) => java_dispatched_command.into_raw(),
        Err(e) => {
            env.throw_new(
                "java/lang/IllegalArgumentException",
                format!("Failed to create DispatchedCommand: {}", e),
            )
            .unwrap();
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn createHubEventCreateDocument(
    mut env: JNIEnv,
    _class: JClass,
    initial_content: jni::sys::jbyteArray,
) -> jobject {
    // Convert byte array to Vec<u8>
    let content_bytes =
        match env.convert_byte_array(jni::objects::JPrimitiveArray::from_raw(initial_content)) {
            Ok(bytes) => bytes,
            Err(e) => {
                env.throw_new(
                    "java/lang/IllegalArgumentException",
                    format!("Failed to convert initial content bytes: {}", e),
                )
                .unwrap();
                return JObject::null().into_raw();
            }
        };

    // Load the Automerge document from the provided bytes
    // Now that versions are aligned, we can load directly
    let automerge_doc = match automerge::Automerge::load(&content_bytes) {
        Ok(doc) => doc,
        Err(e) => {
            env.throw_new(
                "java/lang/IllegalArgumentException",
                format!("Failed to load Automerge document: {}", e),
            )
            .unwrap();
            return JObject::null().into_raw();
        }
    };

    // Create the DispatchedCommand using HubEvent::create_document
    let dispatched_command = HubEvent::create_document(automerge_doc);

    // Convert to Java object
    match dispatched_command_to_java_object(&mut env, dispatched_command) {
        Ok(java_dispatched_command) => java_dispatched_command.into_raw(),
        Err(e) => {
            env.throw_new(
                "java/lang/IllegalArgumentException",
                format!("Failed to create DispatchedCommand: {}", e),
            )
            .unwrap();
            JObject::null().into_raw()
        }
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn createHubEventFindDocument(
    mut env: JNIEnv,
    _class: JClass,
    document_id: jobject,
) -> jobject {
    let doc_id_obj = JObject::from_raw(document_id);
    // Convert DocumentId from Java
    let doc_id = match java_object_to_document_id(&mut env, doc_id_obj) {
        Ok(id) => id,
        Err(e) => {
            env.throw_new(
                "java/lang/IllegalArgumentException",
                format!("Failed to convert DocumentId: {}", e),
            )
            .unwrap();
            return JObject::null().into_raw();
        }
    };

    // Create the DispatchedCommand using HubEvent::find_document
    let dispatched_command = HubEvent::find_document(doc_id);

    // Convert to Java object
    match dispatched_command_to_java_object(&mut env, dispatched_command) {
        Ok(java_dispatched_command) => java_dispatched_command.into_raw(),
        Err(e) => {
            env.throw_new(
                "java/lang/IllegalArgumentException",
                format!("Failed to create DispatchedCommand: {}", e),
            )
            .unwrap();
            JObject::null().into_raw()
        }
    }
}
