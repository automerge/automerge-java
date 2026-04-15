//! Native methods for the ID types under `org.automerge.repo` —
//! `DocumentId`, `PeerId`, `AutomergeUrl`, etc.

use jni::{
    objects::{JByteArray, JClass, JString},
    strings::JNIString,
    NativeMethod,
};

use crate::{interop::throw_illegal_argument, repo::bindings as repo_bindings};

const _METHODS: &[NativeMethod] = &[
    repo_native! { static extern fn document_id_from_bytes(encoded: jbyte[]) -> repo_bindings::DocumentId },
    repo_native! { static extern fn generate_document_id() -> repo_bindings::DocumentId },
    repo_native! { static extern fn generate_peer_id() -> repo_bindings::PeerId },
    repo_native! { static extern fn parse_automerge_url(url: JString) -> repo_bindings::AutomergeUrl },
    repo_native! { static extern fn automerge_url_from_document_id(document_id: repo_bindings::DocumentId) -> JString },
];

// DocumentId ----

fn document_id_from_bytes<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    encoded: JByteArray<'local>,
) -> jni::errors::Result<repo_bindings::DocumentId<'local>> {
    let bytes = env.convert_byte_array(&encoded)?;
    let doc_id = match samod_core::DocumentId::try_from(bytes) {
        Ok(d) => d,
        Err(e) => {
            throw_illegal_argument(
                env,
                &JNIString::from(format!("invalid DocumentId bytes: {}", e)),
            )?;

            return Err(jni::errors::Error::JavaException);
        }
    };
    document_id_to_java(env, &doc_id)
}

fn generate_document_id<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
) -> jni::errors::Result<repo_bindings::DocumentId<'local>> {
    let doc_id = samod_core::DocumentId::new(&mut rand::rng());
    document_id_to_java(env, &doc_id)
}

/// Build the Java [`DocumentId`] wrapper for a samod-core `DocumentId`.
pub(crate) fn document_id_to_java<'local>(
    env: &mut jni::Env<'local>,
    doc_id: &samod_core::DocumentId,
) -> jni::errors::Result<repo_bindings::DocumentId<'local>> {
    let jbytes = env.byte_array_from_slice(doc_id.as_bytes())?;
    repo_bindings::DocumentId::new(env, &jbytes)
}

/// Read a samod-core `DocumentId` out of its Java wrapper.
pub(crate) fn document_id_from_java<'local>(
    env: &mut jni::Env<'local>,
    java_id: &repo_bindings::DocumentId<'local>,
) -> jni::errors::Result<samod_core::DocumentId> {
    let jbytes = java_id.bytes(env)?;
    let bytes = env.convert_byte_array(&jbytes)?;
    match samod_core::DocumentId::try_from(bytes) {
        Ok(d) => Ok(d),
        Err(e) => {
            // Should never happen: the wrapper was constructed from valid bytes.
            env.throw_new(
                jni::jni_str!("java/lang/IllegalStateException"),
                JNIString::from(format!("invalid DocumentId stored in wrapper: {}", e)),
            )?;
            Err(jni::errors::Error::JavaException)
        }
    }
}

// PeerId ----

fn generate_peer_id<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
) -> jni::errors::Result<repo_bindings::PeerId<'local>> {
    let peer_id = samod_core::PeerId::new_with_rng(&mut rand::rng());
    peer_id_to_java(env, &peer_id)
}

pub(crate) fn peer_id_to_java<'local>(
    env: &mut jni::Env<'local>,
    peer_id: &samod_core::PeerId,
) -> jni::errors::Result<repo_bindings::PeerId<'local>> {
    let jstr = env.new_string(peer_id.to_string())?;
    repo_bindings::PeerId::new(env, &jstr)
}

#[allow(dead_code)]
pub(crate) fn peer_id_from_java<'local>(
    env: &mut jni::Env<'local>,
    java_id: &repo_bindings::PeerId<'local>,
) -> jni::errors::Result<samod_core::PeerId> {
    let jstr = java_id.value(env)?;
    let s = jstr.to_string();
    Ok(samod_core::PeerId::from(s))
}

// AutomergeUrl ----

fn parse_automerge_url<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    url_string: JString<'local>,
) -> jni::errors::Result<repo_bindings::AutomergeUrl<'local>> {
    let s = url_string.to_string();
    let parsed: samod_core::AutomergeUrl = match s.parse() {
        Ok(u) => u,
        Err(e) => {
            throw_illegal_argument(
                env,
                &JNIString::from(format!("invalid AutomergeUrl: {}", e)),
            )?;

            return Err(jni::errors::Error::JavaException);
        }
    };
    let doc_id = document_id_to_java(env, parsed.document_id())?;
    repo_bindings::AutomergeUrl::new(env, &doc_id)
}

fn automerge_url_from_document_id<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    document_id: repo_bindings::DocumentId<'local>,
) -> jni::errors::Result<JString<'local>> {
    let doc_id = document_id_from_java(env, &document_id)?;
    let url = samod_core::AutomergeUrl::from(&doc_id);
    env.new_string(url.to_string())
}
