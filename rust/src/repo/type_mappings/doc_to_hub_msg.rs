use jni::objects::{JByteArray, JObject};
use jni::JNIEnv;
use std::convert::TryFrom;

use samod_core::actors::DocToHubMsg;

pub(crate) fn doc_to_hub_msg_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    msg: &DocToHubMsg,
) -> Result<JObject<'local>, jni::errors::Error> {
    // Serialize the message to bytes
    let bytes = msg.to_bytes();

    // Create a Java byte array
    let byte_array = env.byte_array_from_slice(&bytes)?;

    // Create the Java DocToHubMsg object using the package-private constructor
    let doc_to_hub_msg_class = env.find_class(am_classname!("DocToHubMsg"))?;
    let java_obj = env.new_object(doc_to_hub_msg_class, "([B)V", &[(&byte_array).into()])?;

    Ok(java_obj)
}

pub(crate) fn java_object_to_doc_to_hub_msg(
    env: &mut JNIEnv,
    obj: JObject,
) -> Result<DocToHubMsg, jni::errors::Error> {
    // Call the toBytes() method on the Java object
    let bytes_result = env.call_method(&obj, "toBytes", "()[B", &[])?;
    let bytes_obj = bytes_result.l()?;
    let bytes_array: JByteArray = bytes_obj.into();

    // Convert to Rust byte vector
    let bytes = env.convert_byte_array(&bytes_array)?;

    // Deserialize using TryFrom<&[u8]>
    DocToHubMsg::try_from(bytes.as_slice()).map_err(|_| jni::errors::Error::JavaException)
}
