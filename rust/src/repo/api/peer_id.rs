use automerge_jni_macros::jni_fn;
use jni::{sys::jobject, JNIEnv};
use samod_core::PeerId;

use crate::repo::type_mappings::peer_id::peer_id_to_java_object;

/// Generate a new random PeerId
#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn generatePeerId<'local>(env: &mut JNIEnv<'local>) -> jobject {
    // Generate a random PeerId using samod-core's generation logic
    let mut rng = rand::rng(); // is this okay? It'll access a threadlocal RNG
    let peer_id = PeerId::new_with_rng(&mut rng);

    // Convert to Java object
    peer_id_to_java_object(env, &peer_id).unwrap().into_raw()
}
