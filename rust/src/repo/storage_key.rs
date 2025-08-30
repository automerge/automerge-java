// This file contains commented out code for StorageKey implementations
// Keeping it as a placeholder for future development

// #[no_mangle]
// #[jni_fn]
// pub unsafe extern "C" fn createStorageKeyFromParts(
//     mut env: jni::JNIEnv,
//     _class: jni::objects::JClass,
//     parts: jni::sys::jobjectArray,
// ) -> jni::sys::jobject {
//     let parts = JObjectArray::from_raw(parts);

//     // Each part is a string, turn it into a Vec<String> first
//     let part_count = env.get_array_length(&parts).unwrap();
//     let mut parts_vec = Vec::with_capacity(part_count as usize);
//     for i in 0..part_count {
//         let part = env.get_object_array_element(&parts, i).unwrap();
//         let part_jstr = JString::from_raw(part.into_raw());
//         parts_vec.push(String::from(env.get_string(&part_jstr).unwrap()));
//     }

//     // Create a StorageKey from the Vec<String>
//     let storage_key = match StorageKey::from_parts(&parts_vec) {
//         Ok(key) => key,
//         Err(err) => {
//             env.throw_new("java/lang/IllegalArgumentException", &err.to_string())
//                 .unwrap();
//             return JObject::null().into_raw();
//         }
//     };
//     storage_key.to_pointer_obj(&mut env).unwrap()
// }

// #[no_mangle]
// #[jni_fn]
// pub unsafe extern "C" fn storageKeyIsPrefixOf(
//     mut env: jni::JNIEnv,
//     _class: jni::objects::JClass,
//     prefix: jni::sys::jobject,
//     key: jni::sys::jobject,
// ) -> jni::sys::jboolean {
//     let key = StorageKey::from_pointer_obj(&mut env, key).unwrap();
//     let prefix = StorageKey::from_pointer_obj(&mut env, prefix).unwrap();
//     prefix.is_prefix_of(&key).into()
// }

// #[no_mangle]
// #[jni_fn]
// pub unsafe extern "C" fn storageKeyEquals(
//     mut env: jni::JNIEnv,
//     _class: jni::objects::JClass,
//     key: jni::sys::jobject,
//     other: jni::sys::jobject,
// ) -> jni::sys::jboolean {
//     let key = StorageKey::from_pointer_obj(&mut env, key).unwrap();
//     let other = StorageKey::from_pointer_obj(&mut env, other).unwrap();
//     (key == other).into()
// }

// #[no_mangle]
// #[jni_fn]
// pub unsafe extern "C" fn storageKeyHashCode(
//     mut env: jni::JNIEnv,
//     _class: jni::objects::JClass,
//     key: jni::sys::jobject,
// ) -> jni::sys::jint {
//     let key = StorageKey::from_pointer_obj(&mut env, key).unwrap();
//     let mut hasher = std::collections::hash_map::DefaultHasher::new();
//     key.hash(&mut hasher);
//     (hasher.finish() as i32).into()
// }
