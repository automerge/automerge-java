use crate::{
    interop::changehash_to_jobject,
    java_option::make_optional_of,
    repo::type_mappings::{
        connection_id::connection_id_to_java_object, document_id::document_id_to_java_object,
        peer_id::peer_id_to_java_object, timestamp::timestamp_to_java_object,
    },
};
use jni::{
    objects::{JMap, JObject, JValueGen},
    JNIEnv,
};
use samod_core::network::{ConnectionInfo, ConnectionState, PeerDocState};
use std::collections::HashMap;

pub(crate) fn connection_info_to_java<'local>(
    env: &mut JNIEnv<'local>,
    connection_info: &ConnectionInfo,
) -> Result<JObject<'local>, jni::errors::Error> {
    // Convert ConnectionId
    let java_connection_id = connection_id_to_java_object(env, &connection_info.id)?;

    let java_last_received = make_optional_of(env, &connection_info.last_received, |env, ts| {
        timestamp_to_java_object(env, *ts)
    })?;
    let java_last_sent = make_optional_of(env, &connection_info.last_sent, |env, ts| {
        timestamp_to_java_object(env, *ts)
    })?;

    let java_docs = doc_map_to_java(env, &connection_info.docs)?;

    // Convert ConnectionState
    let java_state = connection_state_to_java(env, &connection_info.state)?;

    // Create the Java ConnectionInfo object
    let connection_info_class = env.find_class(am_classname!("ConnectionInfo"))?;
    let connection_info_obj = env.new_object(
        connection_info_class,
        "(Lorg/automerge/ConnectionId;Ljava/util/Optional;Ljava/util/Optional;Ljava/util/Map;Lorg/automerge/ConnectionState;)V",
        &[
            JValueGen::Object(&java_connection_id),
            (&java_last_received).into(),
            (&java_last_sent).into(),
            (&java_docs).into(),
            (&java_state).into(),
        ],
    )?;

    Ok(connection_info_obj)
}

/// Convert a Rust ConnectionState to a Java ConnectionState object
fn connection_state_to_java<'local>(
    env: &mut JNIEnv<'local>,
    state: &ConnectionState,
) -> Result<JObject<'local>, jni::errors::Error> {
    match state {
        ConnectionState::Handshaking => {
            let handshaking_class = env.find_class(am_classname!("ConnectionState$Handshaking"))?;
            let handshaking_obj = env.new_object(handshaking_class, "()V", &[])?;
            Ok(handshaking_obj)
        }
        ConnectionState::Connected { their_peer_id } => {
            let java_peer_id = peer_id_to_java_object(env, their_peer_id)?;
            let connected_class = env.find_class(am_classname!("ConnectionState$Connected"))?;
            let connected_obj = env.new_object(
                connected_class,
                "(Lorg/automerge/PeerId;)V",
                &[(&java_peer_id).into()],
            )?;
            Ok(connected_obj)
        }
    }
}

/// Convert a Rust HashMap<DocumentId, PeerDocState> to a Java Map
fn doc_map_to_java<'local>(
    env: &mut JNIEnv<'local>,
    docs: &HashMap<samod_core::DocumentId, PeerDocState>,
) -> Result<JObject<'local>, jni::errors::Error> {
    // Create a new HashMap
    let hashmap_class = env.find_class("java/util/HashMap")?;
    let hashmap_obj = env.new_object(hashmap_class, "()V", &[])?;
    let java_map = JMap::from_env(env, &hashmap_obj)?;

    // Add each entry to the map
    for (doc_id, peer_doc_state) in docs {
        let java_doc_id = document_id_to_java_object(env, doc_id)?;
        let java_peer_doc_state = peer_doc_state_to_java(env, peer_doc_state)?;

        java_map.put(env, &java_doc_id, &java_peer_doc_state)?;
    }

    Ok(hashmap_obj)
}

/// Convert a Rust PeerDocState to a Java PeerDocState object
fn peer_doc_state_to_java<'local>(
    env: &mut JNIEnv<'local>,
    state: &PeerDocState,
) -> Result<JObject<'local>, jni::errors::Error> {
    // Convert timestamps

    let java_last_received = make_optional_of(env, &state.last_received, |env, ts| {
        timestamp_to_java_object(env, *ts)
    })?;
    let java_last_sent = make_optional_of(env, &state.last_sent, |env, ts| {
        timestamp_to_java_object(env, *ts)
    })?;

    // Convert Optional<Vec<ChangeHash>> to Optional<List<ChangeHash>>
    let java_last_sent_heads = optional_change_hash_vec_to_java(env, &state.last_sent_heads)?;
    let java_last_acked_heads = optional_change_hash_vec_to_java(env, &state.last_acked_heads)?;

    // Create the Java PeerDocState object
    let peer_doc_state_class = env.find_class(am_classname!("PeerDocState"))?;
    let peer_doc_state_obj = env.new_object(
        peer_doc_state_class,
        "(Ljava/util/Optional;Ljava/util/Optional;Ljava/util/Optional;Ljava/util/Optional;)V",
        &[
            (&java_last_received).into(),
            (&java_last_sent).into(),
            (&java_last_sent_heads).into(),
            (&java_last_acked_heads).into(),
        ],
    )?;

    Ok(peer_doc_state_obj)
}

/// Convert a Rust Optional<Vec<ChangeHash>> to a Java Optional<List<ChangeHash>>
fn optional_change_hash_vec_to_java<'local>(
    env: &mut JNIEnv<'local>,
    hashes: &Option<Vec<automerge::ChangeHash>>,
) -> Result<JObject<'local>, jni::errors::Error> {
    match hashes {
        Some(hash_vec) => {
            // Create ArrayList
            let arraylist_class = env.find_class("java/util/ArrayList")?;
            let arraylist_obj = env.new_object(arraylist_class, "()V", &[])?;

            // Add each ChangeHash to the list
            for hash in hash_vec {
                let java_hash = changehash_to_jobject(env, hash)?;
                env.call_method(
                    &arraylist_obj,
                    "add",
                    "(Ljava/lang/Object;)Z",
                    &[(&java_hash).into()],
                )?;
            }

            // Wrap in Optional.of()
            let optional_class = env.find_class("java/util/Optional")?;
            let optional_obj = env.call_static_method(
                optional_class,
                "of",
                "(Ljava/lang/Object;)Ljava/util/Optional;",
                &[(&arraylist_obj).into()],
            )?;

            Ok(optional_obj.l()?)
        }
        None => {
            // Return Optional.empty()
            let optional_class = env.find_class("java/util/Optional")?;
            let empty_optional =
                env.call_static_method(optional_class, "empty", "()Ljava/util/Optional;", &[])?;

            Ok(empty_optional.l()?)
        }
    }
}
