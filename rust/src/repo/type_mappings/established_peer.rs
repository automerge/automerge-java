use crate::repo::type_mappings::{
    connection_id::connection_id_to_java_object, peer_id::peer_id_to_java_object,
};
use jni::{objects::JObject, JNIEnv};
use samod_core::{ConnectionId, PeerId};

/// Convert a Rust (ConnectionId, PeerId) tuple to a Java EstablishedPeer object
pub(crate) fn established_peer_to_java<'local>(
    env: &mut JNIEnv<'local>,
    connection_id: &ConnectionId,
    peer_id: &PeerId,
) -> Result<JObject<'local>, jni::errors::Error> {
    // Convert ConnectionId to Java
    let java_connection_id = connection_id_to_java_object(env, connection_id)?;

    // Convert PeerId to Java
    let java_peer_id = peer_id_to_java_object(env, peer_id)?;

    // Create the Java EstablishedPeer object
    let established_peer_class = env.find_class(am_classname!("EstablishedPeer"))?;
    let established_peer_obj = env.new_object(
        established_peer_class,
        "(Lorg/automerge/ConnectionId;Lorg/automerge/PeerId;)V",
        &[(&java_connection_id).into(), (&java_peer_id).into()],
    )?;

    Ok(established_peer_obj)
}
