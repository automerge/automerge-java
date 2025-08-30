use crate::{
    interop::AsPointerObj,
    repo::type_mappings::{
        doc_actor_result::doc_actor_result_to_java_object, document_id::document_id_to_java_object,
        hub_to_doc_msg::java_object_to_hub_to_doc_msg,
        io_result::java_object_to_io_result_document, spawn_args::java_object_to_spawn_args,
    },
};
use automerge_jni_macros::jni_fn;
use jni::{
    objects::{JClass, JObject},
    sys::{jboolean, jobject},
    JNIEnv,
};
use samod_core::actors::document::DocumentActor;

/// Handle a message with the DocumentActor and return the results
#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn documentActorHandleMsg<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass,
    actor_ptr: jobject,
    timestamp: jni::sys::jlong,
    msg_obj: jobject,
) -> jobject {
    let actor = DocumentActor::from_pointer_obj(&mut env, actor_ptr).unwrap();

    let msg = java_object_to_hub_to_doc_msg(&mut env, JObject::from_raw(msg_obj)).unwrap();

    // Process the message
    let now = samod_core::UnixTimestamp::from_millis(timestamp as u128);
    let result = actor.handle_message(now, msg).unwrap();

    doc_actor_result_to_java_object(&mut env, &result)
        .unwrap()
        .into_raw()
}

/// Execute a function with document access and return the result with side effects
#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn documentActorWithDocument<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass,
    actor_ptr: jobject,
    timestamp: jni::sys::jlong,
    fn_obj: jobject,
) -> jobject {
    let actor = DocumentActor::from_pointer_obj(&mut env, actor_ptr).unwrap();

    // Call with_document with a closure that invokes the Java function
    let now = samod_core::UnixTimestamp::from_millis(timestamp as u128);
    let with_doc_result = actor
        .with_document(now, |doc| {
            // Convert the Rust automerge::Automerge reference to a Java Document
            let doc_ptr = doc as *mut automerge::Automerge as i64;

            // Create a Java Document object wrapping this pointer
            let doc_class = env
                .find_class(am_classname!("AutomergeSys$DocPointer"))
                .unwrap();
            let doc_pointer_obj = env.alloc_object(&doc_class).unwrap();
            env.set_field(&doc_pointer_obj, "pointer", "J", doc_ptr.into())
                .unwrap();

            let document_class = env.find_class(am_classname!("Document")).unwrap();
            let document_obj = env
                .new_object(
                    document_class,
                    "(Lorg/automerge/AutomergeSys$DocPointer;)V",
                    &[(&doc_pointer_obj).into()],
                )
                .unwrap();

            // Call the Java function with the document
            let result = env
                .call_method(
                    JObject::from_raw(fn_obj),
                    "apply",
                    "(Ljava/lang/Object;)Ljava/lang/Object;",
                    &[(&document_obj).into()],
                )
                .unwrap();

            result.l().unwrap()
        })
        .unwrap(); // unwrap the Result<WithDocResult<T>, DocumentError>

    // Convert the result to WithDocResult<T>
    let samod_core::actors::document::WithDocResult {
        value,
        actor_result,
    } = with_doc_result;

    // Convert the DocActorResult to Java
    let actor_result_obj = doc_actor_result_to_java_object(&mut env, &actor_result).unwrap();

    // Create WithDocResult<T> Java object
    let with_doc_result_class = env.find_class(am_classname!("WithDocResult")).unwrap();
    let with_doc_result_obj = env
        .new_object(
            with_doc_result_class,
            "(Ljava/lang/Object;Lorg/automerge/DocActorResult;)V",
            &[(&value).into(), (&actor_result_obj).into()],
        )
        .unwrap();

    with_doc_result_obj.into_raw()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn handleIoComplete<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass,
    actor_ptr: jobject,
    timestamp: jni::sys::jlong,
    io_complete_obj: jobject,
) -> jobject {
    let actor = DocumentActor::from_pointer_obj(&mut env, actor_ptr).unwrap();

    let io_complete_obj = JObject::from_raw(io_complete_obj);
    let io_complete = java_object_to_io_result_document(&mut env, io_complete_obj).unwrap();

    // Process the message
    let now = samod_core::UnixTimestamp::from_millis(timestamp as u128);
    let result = actor.handle_io_complete(now, io_complete).unwrap();

    doc_actor_result_to_java_object(&mut env, &result)
        .unwrap()
        .into_raw()
}

/// Get the document ID from a DocumentActor
#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn documentActorGetDocumentId<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass,
    actor_ptr: jobject,
) -> jobject {
    let actor = DocumentActor::from_pointer_obj(&mut env, actor_ptr).unwrap();

    let document_id = actor.document_id();

    document_id_to_java_object(&mut env, &document_id)
        .unwrap()
        .into_raw()
}

/// Check if the DocumentActor is stopped
#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn documentActorIsStopped<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass,
    actor_ptr: jobject,
) -> jboolean {
    let actor = DocumentActor::from_pointer_obj(&mut env, actor_ptr).unwrap();

    if actor.is_stopped() {
        1
    } else {
        0
    }
}

/// Free a DocumentActor pointer
#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn freeDocumentActor<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass,
    actor_ptr: jobject,
) {
    let _actor = DocumentActor::owned_from_pointer_obj(&mut env, actor_ptr);
    // The DocumentActor will be dropped here, freeing its memory
}

/// Spawn a DocumentActor from SpawnArgs
///
/// Note: This function is typically called internally by the Hub runtime when processing
/// spawn requests. The timestamp and initial DocActorResult should be handled by the caller.
#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn spawnDocumentActor<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass,
    timestamp: jni::sys::jlong,
    spawn_args_obj: jobject,
) -> jobject {
    use java_object_to_spawn_args;

    let spawn_args =
        java_object_to_spawn_args(&mut env, JObject::from_raw(spawn_args_obj)).unwrap();

    // Create the DocumentActor - DocumentActor::new returns (actor, initial_result)
    let now = samod_core::UnixTimestamp::from_millis(timestamp as u128);
    let (actor, _initial_result) = DocumentActor::new(now, spawn_args);

    // Convert to pointer object
    let actor_ptr_jobject = actor.to_pointer_obj(&mut env).unwrap();

    // Create a DocumentActor Java object wrapping this pointer
    let doc_actor_class = env.find_class(am_classname!("DocumentActor")).unwrap();
    let doc_actor_obj = env
        .new_object(
            doc_actor_class,
            "(Lorg/automerge/AutomergeSys$DocumentActorPointer;)V",
            &[(&actor_ptr_jobject).into()],
        )
        .unwrap();

    doc_actor_obj.into_raw()
}
