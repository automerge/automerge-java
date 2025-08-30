use automerge_jni_macros::jni_fn;
use jni::{objects::JValue, sys::jobject, JNIEnv};
use samod_core::DocumentId;

use crate::repo::type_mappings::document_id::DOCUMENT_ID_CLASS;

/// Convert a Rust DocumentId to a Java DocumentId object
#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn documentIdFromBytes(env: &mut JNIEnv<'_>, doc_id: &DocumentId) -> jobject {
    // DocumentId is a wrapper around uuid::Uuid, so we can access the bytes directly
    // doc_id.0.as_bytes() returns &[u8; 16] - the raw UUID bytes
    let uuid_bytes = doc_id.as_bytes(); // 16-byte UUID representation

    // Create Java byte array from UUID bytes
    let java_byte_array = env.byte_array_from_slice(uuid_bytes).unwrap();

    // Create DocumentId object using constructor
    let args = [JValue::from(&java_byte_array)];
    let obj = env.new_object(DOCUMENT_ID_CLASS, "([B)V", &args).unwrap();

    obj.into_raw()
}
