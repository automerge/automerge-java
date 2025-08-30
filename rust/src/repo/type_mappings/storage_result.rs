use jni::objects::{JByteArray, JObject};
use jni::JNIEnv;
use samod_core::io::StorageResult;
use std::collections::HashMap;

use crate::repo::type_mappings::storage_key::java_object_to_storage_key;

pub(crate) fn java_object_to_storage_result(
    env: &mut JNIEnv,
    obj: JObject,
) -> Result<StorageResult, jni::errors::Error> {
    let class_name = env.get_object_class(&obj)?;
    let class_name_str = env.call_method(&class_name, "getName", "()Ljava/lang/String;", &[])?;
    let name_obj = class_name_str.l()?;
    let name_string = env.get_string((&name_obj).into())?;
    let name_str: String = name_string.into();

    match name_str.as_str() {
        "org.automerge.StorageResult$Load" => {
            let value_result = env.call_method(&obj, "getValue", "()Ljava/util/Optional;", &[])?;
            let value_optional = value_result.l()?;

            // Check if Optional is present and extract value
            let is_present = env
                .call_method(&value_optional, "isPresent", "()Z", &[])?
                .z()?;
            let value = if is_present {
                let value_obj = env
                    .call_method(&value_optional, "get", "()Ljava/lang/Object;", &[])?
                    .l()?;
                let value_byte_array: JByteArray = value_obj.into();
                let value_bytes = env.convert_byte_array(&value_byte_array)?;
                Some(value_bytes)
            } else {
                None
            };

            Ok(StorageResult::Load { value })
        }
        "org.automerge.StorageResult$LoadRange" => {
            let values_result = env.call_method(&obj, "getValues", "()Ljava/util/Map;", &[])?;
            let values_map_obj = values_result.l()?;

            // Get the entry set from the Java Map
            let entry_set_result =
                env.call_method(&values_map_obj, "entrySet", "()Ljava/util/Set;", &[])?;
            let entry_set_obj = entry_set_result.l()?;

            // Get iterator from the entry set
            let iterator_result =
                env.call_method(&entry_set_obj, "iterator", "()Ljava/util/Iterator;", &[])?;
            let iterator_obj = iterator_result.l()?;

            let mut values = HashMap::new();

            // Iterate through the entries
            loop {
                let has_next_result = env.call_method(&iterator_obj, "hasNext", "()Z", &[])?;
                if !has_next_result.z()? {
                    break;
                }

                let next_result =
                    env.call_method(&iterator_obj, "next", "()Ljava/lang/Object;", &[])?;
                let entry_obj = next_result.l()?;

                // Get key and value from Map.Entry
                let key_result =
                    env.call_method(&entry_obj, "getKey", "()Ljava/lang/Object;", &[])?;
                let key_obj = key_result.l()?;
                let storage_key = java_object_to_storage_key(env, key_obj)?;

                let value_result =
                    env.call_method(&entry_obj, "getValue", "()Ljava/lang/Object;", &[])?;
                let value_obj = value_result.l()?;
                let value_byte_array: JByteArray = value_obj.into();
                let value_bytes = env.convert_byte_array(&value_byte_array)?;

                values.insert(storage_key, value_bytes);
            }

            Ok(StorageResult::LoadRange { values })
        }
        "org.automerge.StorageResult$Put" => Ok(StorageResult::Put),
        "org.automerge.StorageResult$Delete" => Ok(StorageResult::Delete),
        _ => Err(jni::errors::Error::JavaException),
    }
}
