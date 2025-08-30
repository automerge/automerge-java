use jni::objects::JObject;
use jni::JNIEnv;
use samod_core::StorageId;

pub(crate) const STORAGE_ID_CLASSNAME: &str = am_classname!("StorageId");

pub(crate) fn storage_id_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    storage_id: &StorageId,
) -> Result<JObject<'local>, jni::errors::Error> {
    let storage_id_str = env.new_string(storage_id.to_string())?;
    let storage_id_class = env.find_class(STORAGE_ID_CLASSNAME)?;
    let storage_id_obj = env.new_object(
        storage_id_class,
        "(Ljava/lang/String;)V",
        &[(&storage_id_str).into()],
    )?;
    Ok(storage_id_obj)
}
