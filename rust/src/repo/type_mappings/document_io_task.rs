use crate::repo::type_mappings::peer_id::{java_object_to_peer_id, peer_id_to_java_object};
use crate::repo::type_mappings::storage_task::{
    java_object_to_storage_task, storage_task_to_java_object,
};
use jni::objects::{JObject, JValue};
use jni::JNIEnv;
use samod_core::actors::document::io::DocumentIoTask;

pub(crate) fn document_io_task_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    document_io_task: &DocumentIoTask,
) -> Result<JObject<'local>, jni::errors::Error> {
    match document_io_task {
        DocumentIoTask::Storage(storage_task) => {
            let storage_task_obj = storage_task_to_java_object(env, storage_task)?;

            let class = env.find_class(am_classname!("DocumentIoTask$Storage"))?;
            let obj = env.new_object(
                class,
                "(Lorg/automerge/StorageTask;)V",
                &[JValue::Object(&storage_task_obj)],
            )?;
            Ok(obj)
        }
        DocumentIoTask::CheckAnnouncePolicy { peer_id } => {
            let peer_id_obj = peer_id_to_java_object(env, peer_id)?;

            let class = env.find_class(am_classname!("DocumentIoTask$CheckAnnouncePolicy"))?;
            let obj = env.new_object(
                class,
                "(Lorg/automerge/PeerId;)V",
                &[JValue::Object(&peer_id_obj)],
            )?;
            Ok(obj)
        }
    }
}

pub(crate) fn java_object_to_document_io_task(
    env: &mut JNIEnv,
    obj: JObject,
) -> Result<DocumentIoTask, jni::errors::Error> {
    let class_name = env.get_object_class(&obj)?;
    let class_name = env
        .call_method(&class_name, "getName", "()Ljava/lang/String;", &[])?
        .l()?;
    let class_name_str: String = env.get_string(&class_name.into())?.into();

    match class_name_str.as_str() {
        "org.automerge.DocumentIoTask$Storage" => {
            let storage_task_obj =
                env.get_field(&obj, "storageTask", "Lorg/automerge/StorageTask;")?;
            let storage_task = java_object_to_storage_task(env, storage_task_obj.l()?)?;
            Ok(DocumentIoTask::Storage(storage_task))
        }
        "org.automerge.DocumentIoTask$CheckAnnouncePolicy" => {
            let peer_id_obj = env.get_field(&obj, "peerId", "Lorg/automerge/PeerId;")?;
            let peer_id = java_object_to_peer_id(env, peer_id_obj.l()?)?;
            Ok(DocumentIoTask::CheckAnnouncePolicy { peer_id })
        }
        _ => Err(jni::errors::Error::JavaException),
    }
}
