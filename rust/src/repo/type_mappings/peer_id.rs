use jni::{
    objects::{JObject, JString, JValue},
    JNIEnv,
};
use samod_core::PeerId;

const PEER_ID_CLASS: &str = am_classname!("PeerId");

/// Helper function to convert a Rust PeerId to a Java PeerId object
pub(crate) fn peer_id_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    peer_id: &PeerId,
) -> Result<JObject<'local>, jni::errors::Error> {
    // PeerId is a string wrapper, get the string value
    let peer_string = peer_id.to_string();

    // Create Java string
    let java_string = env.new_string(&peer_string)?;

    // Create PeerId object using constructor
    let args = [JValue::from(&java_string)];
    let obj = env.new_object(PEER_ID_CLASS, "(Ljava/lang/String;)V", &args)?;

    Ok(obj)
}

/// Convert a Java PeerId object to a Rust PeerId (for internal use)
pub(crate) fn java_object_to_peer_id<'local>(
    env: &mut JNIEnv<'local>,
    obj: JObject<'local>,
) -> Result<PeerId, jni::errors::Error> {
    // Get the value field
    let value_field = env.get_field(&obj, "value", "Ljava/lang/String;")?;
    let java_string = JString::from(value_field.l()?);
    let peer_string: String = env.get_string(&java_string)?.into();

    // Create PeerId from string using from_string method
    let peer_id = PeerId::from_string(peer_string);

    Ok(peer_id)
}
