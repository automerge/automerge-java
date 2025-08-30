use jni::{objects::JObject, objects::JValue, JNIEnv};
use samod_core::network::PeerDocState;

/// Convert a Rust PeerDocState to a Java PeerDocState object
pub(crate) fn peer_doc_state_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    peer_doc_state: &PeerDocState,
) -> Result<JObject<'local>, jni::errors::Error> {
    // Convert Optional<UnixTimestamp> to Java Optional<Instant>
    let last_received_obj = if let Some(timestamp) = peer_doc_state.last_received {
        let millis = timestamp.as_millis() as i64;
        let instant_class = env.find_class("java/time/Instant")?;
        let instant_obj = env
            .call_static_method(
                instant_class,
                "ofEpochMilli",
                "(J)Ljava/time/Instant;",
                &[JValue::Long(millis)],
            )?
            .l()?;
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

    let last_sent_obj = if let Some(timestamp) = peer_doc_state.last_sent {
        let millis = timestamp.as_millis() as i64;
        let instant_class = env.find_class("java/time/Instant")?;
        let instant_obj = env
            .call_static_method(
                instant_class,
                "ofEpochMilli",
                "(J)Ljava/time/Instant;",
                &[JValue::Long(millis)],
            )?
            .l()?;
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

    // Convert Optional<Vec<ChangeHash>> to Java Optional<List<ChangeHash>>
    let last_sent_heads_obj = if let Some(heads) = &peer_doc_state.last_sent_heads {
        let array_list_class = env.find_class("java/util/ArrayList")?;
        let list_obj = env.new_object(array_list_class, "()V", &[])?;

        for change_hash in heads {
            let change_hash_obj = change_hash_to_java_object(env, change_hash)?;
            env.call_method(
                &list_obj,
                "add",
                "(Ljava/lang/Object;)Z",
                &[JValue::Object(&change_hash_obj)],
            )?;
        }

        // Create Optional.of(list)
        let optional_class = env.find_class("java/util/Optional")?;
        env.call_static_method(
            optional_class,
            "of",
            "(Ljava/lang/Object;)Ljava/util/Optional;",
            &[JValue::Object(&list_obj)],
        )?
        .l()?
    } else {
        // Create Optional.empty()
        let optional_class = env.find_class("java/util/Optional")?;
        env.call_static_method(optional_class, "empty", "()Ljava/util/Optional;", &[])?
            .l()?
    };

    let last_acked_heads_obj = if let Some(heads) = &peer_doc_state.last_acked_heads {
        let array_list_class = env.find_class("java/util/ArrayList")?;
        let list_obj = env.new_object(array_list_class, "()V", &[])?;

        for change_hash in heads {
            let change_hash_obj = change_hash_to_java_object(env, change_hash)?;
            env.call_method(
                &list_obj,
                "add",
                "(Ljava/lang/Object;)Z",
                &[JValue::Object(&change_hash_obj)],
            )?;
        }

        // Create Optional.of(list)
        let optional_class = env.find_class("java/util/Optional")?;
        env.call_static_method(
            optional_class,
            "of",
            "(Ljava/lang/Object;)Ljava/util/Optional;",
            &[JValue::Object(&list_obj)],
        )?
        .l()?
    } else {
        // Create Optional.empty()
        let optional_class = env.find_class("java/util/Optional")?;
        env.call_static_method(optional_class, "empty", "()Ljava/util/Optional;", &[])?
            .l()?
    };

    // Create PeerDocState object
    let class = env.find_class(am_classname!("PeerDocState"))?;
    let obj = env.new_object(
        class,
        "(Ljava/util/Optional;Ljava/util/Optional;Ljava/util/Optional;Ljava/util/Optional;)V",
        &[
            JValue::Object(&last_received_obj),
            JValue::Object(&last_sent_obj),
            JValue::Object(&last_sent_heads_obj),
            JValue::Object(&last_acked_heads_obj),
        ],
    )?;

    Ok(obj)
}

/// Helper function to convert a Rust ChangeHash to a Java ChangeHash object
fn change_hash_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    change_hash: &automerge::ChangeHash,
) -> Result<JObject<'local>, jni::errors::Error> {
    // ChangeHash is represented as byte array in Java
    let bytes = change_hash.as_ref();
    let byte_array = env.byte_array_from_slice(bytes)?;

    let class = env.find_class(am_classname!("ChangeHash"))?;
    let obj = env.new_object(class, "([B)V", &[JValue::Object(&byte_array)])?;
    Ok(obj)
}

/// Helper function to convert a Java List<ChangeHash> to a Rust Vec<ChangeHash>
fn java_list_to_change_hash_vec(
    env: &mut JNIEnv,
    list_obj: JObject,
) -> Result<Vec<automerge::ChangeHash>, jni::errors::Error> {
    let size = env.call_method(&list_obj, "size", "()I", &[])?.i()?;
    let mut result = Vec::new();

    for i in 0..size {
        let item_obj = env
            .call_method(&list_obj, "get", "(I)Ljava/lang/Object;", &[JValue::Int(i)])?
            .l()?;
        let change_hash = java_object_to_change_hash(env, item_obj)?;
        result.push(change_hash);
    }

    Ok(result)
}

/// Helper function to convert a Java ChangeHash object to a Rust ChangeHash
fn java_object_to_change_hash(
    env: &mut JNIEnv,
    obj: JObject,
) -> Result<automerge::ChangeHash, jni::errors::Error> {
    // Get the bytes from the ChangeHash object
    let bytes_result = env.call_method(&obj, "getBytes", "()[B", &[])?;
    let bytes_obj = bytes_result.l()?;
    let byte_array = jni::objects::JByteArray::from(bytes_obj);
    let bytes = env.convert_byte_array(&byte_array)?;

    // Convert to ChangeHash
    automerge::ChangeHash::try_from(bytes.as_slice()).map_err(|_| jni::errors::Error::JavaException)
}
