use jni::{
    objects::{JObject, JObjectArray},
    JNIEnv,
};
use samod_core::StorageKey;

pub(crate) const STORAGE_KEY_CLASS_NAME: &str = am_classname!("StorageKey");

pub(crate) fn storage_key_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    storage_key: &StorageKey,
) -> Result<JObject<'local>, jni::errors::Error> {
    // Use IntoIterator to get the parts directly
    let parts: Vec<String> = storage_key.into_iter().cloned().collect();

    let string_class = env.find_class("java/lang/String")?;
    let parts_array = env.new_object_array(parts.len() as i32, string_class, JObject::null())?;

    for (i, part) in parts.iter().enumerate() {
        let part_string = env.new_string(part)?;
        env.set_object_array_element(&parts_array, i as i32, part_string)?;
    }

    let storage_key_class = env.find_class(STORAGE_KEY_CLASS_NAME)?;
    let storage_key_obj = env.new_object(
        storage_key_class,
        "([Ljava/lang/String;)V",
        &[(&parts_array).into()],
    )?;
    Ok(storage_key_obj)
}

pub(crate) fn java_object_to_storage_key(
    env: &mut JNIEnv,
    obj: JObject,
) -> Result<StorageKey, jni::errors::Error> {
    // Get the parts field from StorageKey using correct JNI API
    let parts_array = env.get_field(&obj, "parts", "[Ljava/lang/String;")?;
    let parts_array_obj = parts_array.l()?;

    let parts_array_ref: JObjectArray = parts_array_obj.into();
    let array_length = env.get_array_length(&parts_array_ref)?;
    let mut parts = Vec::with_capacity(array_length as usize);

    for i in 0..array_length {
        let element = env.get_object_array_element(&parts_array_ref, i)?;
        let string_obj = env.get_string((&element).into())?;
        let part: String = string_obj.into();
        parts.push(part);
    }

    StorageKey::from_parts(&parts).map_err(|_| jni::errors::Error::JavaException)
}
