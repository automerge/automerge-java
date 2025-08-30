use automerge_jni_macros::jni_fn;
use jni::{
    objects::{JByteArray, JClass, JObject},
    sys::{jbyteArray, jobject, jstring},
    JNIEnv,
};
use samod_core::actors::document::SpawnArgs;

use crate::repo::type_mappings::spawn_args::java_object_to_spawn_args;

/// Create a SpawnArgs from serialized bytes
/// This throws IllegalArgumentException if the bytes are invalid
#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn createSpawnArgsFromBytes<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass,
    bytes: jbyteArray,
) -> jobject {
    let bytes = JByteArray::from_raw(bytes);
    let byte_vec = match env.convert_byte_array(bytes) {
        Ok(bytes) => bytes,
        Err(e) => {
            env.throw_new("java/lang/IllegalArgumentException", e.to_string())
                .unwrap();
            return JObject::null().into_raw();
        }
    };

    // Try to deserialize the bytes into a SpawnArgs
    let _spawn_args = match SpawnArgs::try_from(byte_vec.as_slice()) {
        Ok(args) => args,
        Err(e) => {
            env.throw_new(
                "java/lang/IllegalArgumentException",
                format!("Invalid SpawnArgs bytes: {:?}", e),
            )
            .unwrap();
            return JObject::null().into_raw();
        }
    };

    // Create the Java object directly using the package-private constructor
    let spawn_args_class = match env.find_class(am_classname!("SpawnArgs")) {
        Ok(class) => class,
        Err(e) => {
            env.throw_new("java/lang/IllegalArgumentException", e.to_string())
                .unwrap();
            return JObject::null().into_raw();
        }
    };

    let byte_array = match env.byte_array_from_slice(&byte_vec) {
        Ok(array) => array,
        Err(e) => {
            env.throw_new("java/lang/IllegalArgumentException", e.to_string())
                .unwrap();
            return JObject::null().into_raw();
        }
    };

    let java_obj = match env.new_object(spawn_args_class, "([B)V", &[(&byte_array).into()]) {
        Ok(obj) => obj,
        Err(e) => {
            env.throw_new("java/lang/IllegalArgumentException", e.to_string())
                .unwrap();
            return JObject::null().into_raw();
        }
    };

    java_obj.into_raw()
}

/// Display a SpawnArgs using the Rust Display implementation
#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn displaySpawnArgs<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass,
    spawn_args_obj: JObject,
) -> jstring {
    // Convert Java object to Rust SpawnArgs
    let spawn_args = match java_object_to_spawn_args(&mut env, spawn_args_obj) {
        Ok(args) => args,
        Err(e) => {
            env.throw_new("java/lang/IllegalArgumentException", e.to_string())
                .unwrap();
            return JObject::null().into_raw();
        }
    };

    // Use the Display implementation
    let display_string = format!("{:?}", spawn_args);

    // Convert to Java string
    match env.new_string(display_string) {
        Ok(jstr) => jstr.into_raw(),
        Err(e) => {
            env.throw_new("java/lang/IllegalArgumentException", e.to_string())
                .unwrap();
            JObject::null().into_raw()
        }
    }
}
