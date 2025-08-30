use jni::objects::{JByteArray, JObject};
use jni::JNIEnv;
use std::convert::TryFrom;

use samod_core::actors::HubToDocMsg;

/// Convert a Rust HubToDocMsg to a Java HubToDocMsg object
pub(crate) fn hub_to_doc_msg_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    msg: &HubToDocMsg,
) -> Result<JObject<'local>, jni::errors::Error> {
    // Serialize the message to bytes
    let bytes = msg.to_bytes();

    // Create a Java byte array
    let byte_array = env.byte_array_from_slice(&bytes)?;

    // Create the Java HubToDocMsg object using the private constructor
    let hub_to_doc_msg_class = env.find_class(am_classname!("HubToDocMsg"))?;
    let java_obj = env.new_object(hub_to_doc_msg_class, "([B)V", &[(&byte_array).into()])?;

    Ok(java_obj)
}

/// Convert a Java HubToDocMsg object to a Rust HubToDocMsg
pub(crate) fn java_object_to_hub_to_doc_msg(
    env: &mut JNIEnv,
    obj: JObject,
) -> Result<HubToDocMsg, jni::errors::Error> {
    // Call the toBytes() method on the Java object
    let bytes_result = env.call_method(&obj, "toBytes", "()[B", &[])?;
    let bytes_obj = bytes_result.l()?;
    let bytes_array: JByteArray = bytes_obj.into();

    // Convert to Rust byte vector
    let bytes = env.convert_byte_array(&bytes_array)?;

    // Deserialize using TryFrom<&[u8]>
    HubToDocMsg::try_from(bytes.as_slice()).map_err(|_| jni::errors::Error::JavaException)
}
