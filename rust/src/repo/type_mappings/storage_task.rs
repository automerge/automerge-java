use jni::objects::{JByteArray, JObject};
use jni::JNIEnv;
use samod_core::io::StorageTask;

use crate::repo::type_mappings::storage_key::{
    java_object_to_storage_key, storage_key_to_java_object, STORAGE_KEY_CLASS_NAME,
};

pub(crate) const STORAGE_TASK_CLASS_NAME: &str = am_classname!("StorageTask");
pub(crate) const STORAGE_TASK_LOAD_CLASSNAME: &str = am_classname!("StorageTask$Load");
pub(crate) const STORAGE_TASK_LOAD_RANGE_CLASSNAME: &str = am_classname!("StorageTask$LoadRange");
pub(crate) const STORAGE_TASK_PUT_CLASSNAME: &str = am_classname!("StorageTask$Put");
pub(crate) const STORAGE_TASK_DELETE_CLASSNAME: &str = am_classname!("StorageTask$Delete");

pub(crate) fn storage_task_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    storage_task: &StorageTask,
) -> Result<JObject<'local>, jni::errors::Error> {
    match storage_task {
        StorageTask::Load { key } => {
            let key_obj = storage_key_to_java_object(env, key)?;
            let load_class = env.find_class(STORAGE_TASK_LOAD_CLASSNAME)?;
            let load_obj = env.new_object(
                load_class,
                format!("(L{};)V", STORAGE_KEY_CLASS_NAME),
                &[(&key_obj).into()],
            )?;
            Ok(load_obj)
        }
        StorageTask::LoadRange { prefix } => {
            let prefix_obj = storage_key_to_java_object(env, prefix)?;
            let load_range_class = env.find_class(STORAGE_TASK_LOAD_RANGE_CLASSNAME)?;
            let load_range_obj = env.new_object(
                load_range_class,
                format!("(L{};)V", STORAGE_KEY_CLASS_NAME),
                &[(&prefix_obj).into()],
            )?;
            Ok(load_range_obj)
        }
        StorageTask::Put { key, value } => {
            let key_obj = storage_key_to_java_object(env, key)?;
            let value_array = env.byte_array_from_slice(value)?;
            let put_class = env.find_class(STORAGE_TASK_PUT_CLASSNAME)?;
            let put_obj = env.new_object(
                put_class,
                format!("(L{};[B)V", STORAGE_KEY_CLASS_NAME),
                &[(&key_obj).into(), (&value_array).into()],
            )?;
            Ok(put_obj)
        }
        StorageTask::Delete { key } => {
            let key_obj = storage_key_to_java_object(env, key)?;
            let delete_class = env.find_class(STORAGE_TASK_DELETE_CLASSNAME)?;
            let delete_obj = env.new_object(
                delete_class,
                format!("(L{};)V", STORAGE_KEY_CLASS_NAME),
                &[(&key_obj).into()],
            )?;
            Ok(delete_obj)
        }
    }
}

pub(crate) fn java_object_to_storage_task<'local>(
    env: &mut JNIEnv<'local>,
    obj: JObject<'local>,
) -> Result<StorageTask, jni::errors::Error> {
    let class_name = env.get_object_class(&obj)?;
    let class_name_str = env.call_method(&class_name, "getName", "()Ljava/lang/String;", &[])?;
    let name_obj = class_name_str.l()?;
    let name_string = env.get_string((&name_obj).into())?;
    let name_str: String = name_string.into();
    let final_name_component = name_str.split('/').last().unwrap_or_default();
    let inner_class_name = final_name_component.split("$").next().unwrap_or_default();

    match inner_class_name {
        "Load" => {
            let key_result = env.call_method(
                &obj,
                "getKey",
                format!("()L{};", STORAGE_KEY_CLASS_NAME),
                &[],
            )?;
            let key_obj = key_result.l()?;
            let key = java_object_to_storage_key(env, key_obj)?;
            Ok(StorageTask::Load { key })
        }
        "LoadRange" => {
            let prefix_result = env.call_method(
                &obj,
                "getPrefix",
                format!("()L{};", STORAGE_KEY_CLASS_NAME),
                &[],
            )?;
            let prefix_obj = prefix_result.l()?;
            let prefix = java_object_to_storage_key(env, prefix_obj)?;
            Ok(StorageTask::LoadRange { prefix })
        }
        "Put" => {
            let key_result = env.call_method(
                &obj,
                "getKey",
                format!("()L{};", STORAGE_KEY_CLASS_NAME),
                &[],
            )?;
            let key_obj = key_result.l()?;
            let key = java_object_to_storage_key(env, key_obj)?;

            let value_result = env.call_method(&obj, "getValue", "()[B", &[])?;
            let value_array = value_result.l()?;
            let value_byte_array: JByteArray = value_array.into();
            let value_bytes = env.convert_byte_array(&value_byte_array)?;

            Ok(StorageTask::Put {
                key,
                value: value_bytes,
            })
        }
        "Delete" => {
            let key_result = env.call_method(
                &obj,
                "getKey",
                format!("()L{};", STORAGE_KEY_CLASS_NAME),
                &[],
            )?;
            let key_obj = key_result.l()?;
            let key = java_object_to_storage_key(env, key_obj)?;
            Ok(StorageTask::Delete { key })
        }
        _ => Err(jni::errors::Error::JavaException),
    }
}
