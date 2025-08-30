use automerge_jni_macros::jni_fn;
use jni::{
    objects::{JClass, JObject},
    sys::{jboolean, jlong, jobject},
    JNIEnv,
};
use samod_core::{actors::hub::Hub, UnixTimestamp};

use crate::{
    interop::AsPointerObj,
    repo::type_mappings::{
        connection_event::connection_info_to_java_object,
        established_peer::established_peer_to_java,
        hub_results::hub_results_to_java_object,
        peer_id::{java_object_to_peer_id, peer_id_to_java_object},
        storage_id::storage_id_to_java_object,
    },
};

/// Handle an event with the Hub and return the results
#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn hubHandleEvent(
    mut env: JNIEnv,
    _class: JClass,
    hub_ptr: jobject,
    timestamp: jlong,
    event_ptr: jobject,
) -> jobject {
    let hub = Hub::from_pointer_obj(&mut env, hub_ptr).unwrap();

    let event =
        samod_core::actors::hub::HubEvent::owned_from_pointer_obj(&mut env, event_ptr).unwrap();

    let timestamp = UnixTimestamp::from_millis(timestamp as u128);

    // Generate a random number generator for the hub
    let mut rng = rand::rng();

    // Process the event
    let results = hub.handle_event(&mut rng, timestamp, *event);

    hub_results_to_java_object(&mut env, &results)
        .unwrap()
        .into_raw()
}

/// Get the peer ID from a Hub
#[no_mangle]
pub unsafe extern "C" fn hubGetPeerId(
    mut env: JNIEnv,
    _class: JClass,
    hub_ptr: jobject,
) -> jobject {
    let hub = Hub::from_pointer_obj(&mut env, hub_ptr).unwrap();

    let peer_id = hub.peer_id();

    peer_id_to_java_object(&mut env, &peer_id)
        .unwrap()
        .into_raw()
}

/// Get the storage ID from a Hub
#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn hubGetStorageId(
    mut env: JNIEnv,
    _class: JClass,
    hub_ptr: jobject,
) -> jobject {
    let hub = Hub::from_pointer_obj(&mut env, hub_ptr).unwrap();

    let storage_id = hub.storage_id();
    storage_id_to_java_object(&mut env, &storage_id)
        .unwrap()
        .into_raw()
}

/// Get all connections from a Hub
#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn hubGetConnections(
    mut env: JNIEnv,
    _class: JClass,
    hub_ptr: jobject,
) -> jobject {
    let hub = Hub::from_pointer_obj(&mut env, hub_ptr).unwrap();

    let connections = hub.connections();

    // Convert Vec<ConnectionInfo> to Java ConnectionInfo[]
    let connection_class = env.find_class(am_classname!("ConnectionInfo")).unwrap();

    let java_array = env
        .new_object_array(connections.len() as i32, &connection_class, JObject::null())
        .unwrap();

    for (i, connection_info) in connections.iter().enumerate() {
        let java_connection_info =
            connection_info_to_java_object(&mut env, connection_info).unwrap();

        env.set_object_array_element(&java_array, i as i32, java_connection_info)
            .unwrap();
    }

    java_array.into_raw()
}

/// Get all established peers from a Hub
#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn hubGetEstablishedPeers(
    mut env: JNIEnv,
    _class: JClass,
    hub_ptr: jobject,
) -> jobject {
    let hub = Hub::from_pointer_obj(&mut env, hub_ptr).unwrap();

    let established_peers = hub.established_peers();

    // Convert Vec<(ConnectionId, PeerId)> to Java EstablishedPeer[]
    let established_peer_class = env.find_class(am_classname!("EstablishedPeer")).unwrap();

    let java_array = env
        .new_object_array(
            established_peers.len() as i32,
            &established_peer_class,
            JObject::null(),
        )
        .unwrap();

    for (i, (connection_id, peer_id)) in established_peers.iter().enumerate() {
        let java_established_peer =
            established_peer_to_java(&mut env, connection_id, peer_id).unwrap();

        env.set_object_array_element(&java_array, i as i32, java_established_peer)
            .unwrap();
    }

    java_array.into_raw()
}

/// Check if the Hub is connected to a specific peer
#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn hubIsConnectedTo(
    mut env: JNIEnv,
    _class: JClass,
    hub_ptr: jobject,
    peer_id_obj: jobject,
) -> jboolean {
    let hub = Hub::from_pointer_obj(&mut env, hub_ptr).unwrap();

    let peer_id = java_object_to_peer_id(&mut env, JObject::from_raw(peer_id_obj)).unwrap();

    if hub.is_connected_to(&peer_id) {
        1
    } else {
        0
    }
}

/// Check if the Hub is stopped
#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn hubIsStopped(
    mut env: JNIEnv,
    _class: JClass,
    hub_ptr: jobject,
) -> jboolean {
    let hub = Hub::from_pointer_obj(&mut env, hub_ptr).unwrap();

    if hub.is_stopped() {
        1
    } else {
        0
    }
}

/// Free a Hub pointer
#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn freeHub(mut env: JNIEnv, _class: JClass, hub_ptr: jobject) {
    let _hub = Hub::owned_from_pointer_obj(&mut env, hub_ptr);
    // The Hub will be dropped here, freeing its memory
}
