use crate::{java_option::make_optional_of, repo::type_mappings::peer_id::peer_id_to_java_object};
use jni::{objects::JObject, objects::JValue, JNIEnv};
use samod_core::network::{PeerInfo, PeerMetadata};

/// Convert a Rust PeerMetadata to a Java PeerMetadata object
pub(crate) fn peer_metadata_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    peer_metadata: &PeerMetadata,
) -> Result<JObject<'local>, jni::errors::Error> {
    // Convert boolean to JValue
    let is_ephemeral = JValue::Bool(peer_metadata.is_ephemeral as jni::sys::jboolean);

    // Create PeerMetadata object using constructor (boolean)
    let peer_metadata_obj =
        env.new_object(am_classname!("PeerMetadata"), "(Z)V", &[is_ephemeral])?;

    Ok(peer_metadata_obj)
}

/// Convert a Rust PeerInfo to a Java PeerInfo object
pub(crate) unsafe fn peer_info_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    peer_info: &PeerInfo,
) -> Result<JObject<'local>, jni::errors::Error> {
    // Convert PeerId to Java
    let peer_id_java = peer_id_to_java_object(env, &peer_info.peer_id)?;

    // Convert optional PeerMetadata to Java Optional
    let metadata_optional =
        make_optional_of(env, &peer_info.metadata, peer_metadata_to_java_object)?;

    // Convert protocol version String to Java
    let protocol_version_java = env.new_string(&peer_info.protocol_version)?;

    // Create PeerInfo object using constructor (PeerId, Optional<PeerMetadata>, String)
    let peer_info_obj = env.new_object(
        am_classname!("PeerInfo"),
        "(Lorg/automerge/PeerId;Ljava/util/Optional;Ljava/lang/String;)V",
        &[
            JValue::Object(&peer_id_java),
            JValue::Object(&metadata_optional),
            JValue::Object(&protocol_version_java),
        ],
    )?;

    Ok(peer_info_obj)
}
