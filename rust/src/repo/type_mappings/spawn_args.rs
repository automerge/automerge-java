use jni::objects::{JByteArray, JObject};
use jni::JNIEnv;
use std::convert::TryFrom;

use samod_core::actors::document::SpawnArgs;

/// Convert a Rust SpawnArgs to a Java SpawnArgs object
pub(crate) fn spawn_args_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    spawn_args: &SpawnArgs,
) -> Result<JObject<'local>, jni::errors::Error> {
    // Serialize the spawn args to bytes
    let bytes = spawn_args.to_bytes();

    // Create a Java byte array
    let byte_array = env.byte_array_from_slice(&bytes)?;

    // Create the Java SpawnArgs object using the package-private constructor
    let spawn_args_class = env.find_class(am_classname!("SpawnArgs"))?;
    let java_obj = env.new_object(spawn_args_class, "([B)V", &[(&byte_array).into()])?;

    Ok(java_obj)
}

/// Convert a Java SpawnArgs object to a Rust SpawnArgs
pub(crate) fn java_object_to_spawn_args(
    env: &mut JNIEnv,
    obj: JObject,
) -> Result<SpawnArgs, jni::errors::Error> {
    // Call the toBytes() method on the Java object
    let bytes_result = env.call_method(&obj, "toBytes", "()[B", &[])?;
    let bytes_obj = bytes_result.l()?;
    let bytes_array: JByteArray = bytes_obj.into();

    // Convert to Rust byte vector
    let bytes = env.convert_byte_array(&bytes_array)?;

    // Deserialize using TryFrom<&[u8]>
    SpawnArgs::try_from(bytes.as_slice()).map_err(|_| jni::errors::Error::JavaException)
}
