use jni::objects::JObject;
use jni::JNIEnv;
use samod_core::actors::document::io::DocumentIoResult;

use crate::repo::type_mappings::storage_result::java_object_to_storage_result;

pub(crate) fn java_object_to_document_io_result(
    env: &mut JNIEnv,
    obj: JObject,
) -> Result<DocumentIoResult, jni::errors::Error> {
    let class_name = env.get_object_class(&obj)?;
    let class_name = env
        .call_method(&class_name, "getName", "()Ljava/lang/String;", &[])?
        .l()?;
    let class_name_str: String = env.get_string(&class_name.into())?.into();

    match class_name_str.as_str() {
        "org.automerge.DocumentIoResult$Storage" => {
            let storage_result_obj = env.get_field(
                &obj,
                "storageResult",
                format!("L{};", am_classname!("StorageResult")),
            )?;
            let storage_result = java_object_to_storage_result(env, storage_result_obj.l()?)?;
            Ok(DocumentIoResult::Storage(storage_result))
        }
        "org.automerge.DocumentIoResult$CheckAnnouncePolicy" => {
            let should_announce = env.get_field(&obj, "shouldAnnounce", "Z")?.z()?;
            Ok(DocumentIoResult::CheckAnnouncePolicy(should_announce))
        }
        _ => Err(jni::errors::Error::JavaException),
    }
}
