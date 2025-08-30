use crate::repo::type_mappings::{
    connection_id::connection_id_to_java_object,
    document_id::document_id_to_java_object,
    peer_doc_state::peer_doc_state_to_java_object,
    peer_id::{java_object_to_peer_id, peer_id_to_java_object},
    peer_info::peer_info_to_java_object,
    timestamp::timestamp_to_java_object,
};
use jni::{
    objects::{JObject, JValue},
    JNIEnv,
};
use samod_core::network::{ConnectionEvent, ConnectionInfo, ConnectionState};

pub(crate) fn connection_event_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    connection_event: &ConnectionEvent,
) -> Result<JObject<'local>, jni::errors::Error> {
    match connection_event {
        ConnectionEvent::HandshakeCompleted {
            connection_id,
            peer_info,
        } => {
            let connection_id_obj = connection_id_to_java_object(env, connection_id)?;
            let peer_info_obj = unsafe { peer_info_to_java_object(env, peer_info)? };

            let class = env.find_class(am_classname!("ConnectionEvent$HandshakeCompleted"))?;
            let obj = env.new_object(
                class,
                "(Lorg/automerge/ConnectionId;Lorg/automerge/PeerInfo;)V",
                &[
                    JValue::Object(&connection_id_obj),
                    JValue::Object(&peer_info_obj),
                ],
            )?;
            Ok(obj)
        }
        ConnectionEvent::ConnectionFailed {
            connection_id,
            error,
        } => {
            let connection_id_obj = connection_id_to_java_object(env, connection_id)?;
            let error_jstring = env.new_string(error)?;

            let class = env.find_class(am_classname!("ConnectionEvent$ConnectionFailed"))?;
            let obj = env.new_object(
                class,
                "(Lorg/automerge/ConnectionId;Ljava/lang/String;)V",
                &[
                    JValue::Object(&connection_id_obj),
                    JValue::Object(&error_jstring),
                ],
            )?;
            Ok(obj)
        }
        ConnectionEvent::StateChanged {
            connection_id,
            new_state,
        } => {
            let connection_id_obj = connection_id_to_java_object(env, connection_id)?;
            let new_state_obj = connection_info_to_java_object(env, new_state)?;

            let class = env.find_class(am_classname!("ConnectionEvent$StateChanged"))?;
            let obj = env.new_object(
                class,
                "(Lorg/automerge/ConnectionId;Lorg/automerge/ConnectionInfo;)V",
                &[
                    JValue::Object(&connection_id_obj),
                    JValue::Object(&new_state_obj),
                ],
            )?;
            Ok(obj)
        }
    }
}

pub(crate) fn connection_info_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    connection_info: &ConnectionInfo,
) -> Result<JObject<'local>, jni::errors::Error> {
    let connection_id_obj = connection_id_to_java_object(env, &connection_info.id)?;

    // Convert Optional<UnixTimestamp> to Java Optional<Instant>
    let last_received_obj = if let Some(timestamp) = connection_info.last_received {
        let instant_obj = timestamp_to_java_object(env, timestamp)?;
        let optional_class = env.find_class("java/util/Optional")?;
        env.call_static_method(
            optional_class,
            "of",
            "(Ljava/lang/Object;)Ljava/util/Optional;",
            &[JValue::Object(&instant_obj)],
        )?
        .l()?
    } else {
        // Create Optional.empty()
        let optional_class = env.find_class("java/util/Optional")?;
        env.call_static_method(optional_class, "empty", "()Ljava/util/Optional;", &[])?
            .l()?
    };

    let last_sent_obj = if let Some(timestamp) = connection_info.last_sent {
        let instant_obj = timestamp_to_java_object(env, timestamp)?;
        // Create Optional.of(instant)
        let optional_class = env.find_class("java/util/Optional")?;
        env.call_static_method(
            optional_class,
            "of",
            "(Ljava/lang/Object;)Ljava/util/Optional;",
            &[JValue::Object(&instant_obj)],
        )?
        .l()?
    } else {
        // Create Optional.empty()
        let optional_class = env.find_class("java/util/Optional")?;
        env.call_static_method(optional_class, "empty", "()Ljava/util/Optional;", &[])?
            .l()?
    };

    // Convert docs HashMap<DocumentId, PeerDocState> to Java
    let hashmap_class = env.find_class("java/util/HashMap")?;
    let docs_obj = env.new_object(hashmap_class, "()V", &[])?;

    for (doc_id, peer_doc_state) in &connection_info.docs {
        let doc_id_obj = document_id_to_java_object(env, doc_id)?;
        let peer_doc_state_obj = peer_doc_state_to_java_object(env, peer_doc_state)?;

        env.call_method(
            &docs_obj,
            "put",
            "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
            &[
                JValue::Object(&doc_id_obj),
                JValue::Object(&peer_doc_state_obj),
            ],
        )?;
    }

    let state_obj = connection_state_to_java_object(env, &connection_info.state)?;

    let class = env.find_class(am_classname!("ConnectionInfo"))?;
    let obj = env.new_object(
        class,
        "(Lorg/automerge/ConnectionId;Ljava/util/Optional;Ljava/util/Optional;Ljava/util/Map;Lorg/automerge/ConnectionState;)V",
        &[
            JValue::Object(&connection_id_obj),
            JValue::Object(&last_received_obj),
            JValue::Object(&last_sent_obj),
            JValue::Object(&docs_obj),
            JValue::Object(&state_obj),
        ],
    )?;
    Ok(obj)
}

pub(crate) fn connection_state_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    connection_state: &ConnectionState,
) -> Result<JObject<'local>, jni::errors::Error> {
    match connection_state {
        ConnectionState::Handshaking => {
            let class = env.find_class(am_classname!("ConnectionState$Handshaking"))?;
            let obj = env.new_object(class, "()V", &[])?;
            Ok(obj)
        }
        ConnectionState::Connected { their_peer_id } => {
            let peer_id_obj = peer_id_to_java_object(env, their_peer_id)?;
            let class = env.find_class(am_classname!("ConnectionState$Connected"))?;
            let obj =
                env.new_object(class, "(Lorg/automerge/PeerId;)V", &[(&peer_id_obj).into()])?;
            Ok(obj)
        }
    }
}

pub(crate) fn java_object_to_connection_state<'local>(
    env: &mut JNIEnv<'local>,
    obj: JObject<'local>,
) -> Result<ConnectionState, jni::errors::Error> {
    let handshaking_class = env.find_class(am_classname!("ConnectionState$Handshaking"))?;
    let connected_class = env.find_class(am_classname!("ConnectionState$Connected"))?;

    if env.is_instance_of(&obj, handshaking_class)? {
        Ok(ConnectionState::Handshaking)
    } else if env.is_instance_of(&obj, connected_class)? {
        let their_peer_id_field = env.get_field(&obj, "theirPeerId", "Lorg/automerge/PeerId;")?;
        let their_peer_id_obj = their_peer_id_field.l()?;
        let their_peer_id = java_object_to_peer_id(env, their_peer_id_obj)?;
        Ok(ConnectionState::Connected { their_peer_id })
    } else {
        Err(jni::errors::Error::JavaException)
    }
}
