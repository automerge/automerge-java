use automerge::{ActorId, Automerge, PatchLog};
use automerge_jni_macros::jni_fn;
use jni::{
    objects::{JObject, JPrimitiveArray},
    sys::{jbyteArray, jobject},
};

use crate::{
    interop::{heads_from_jobject, throw_amg_exc_or_fatal, JavaPointer},
    patches::to_patch_arraylist,
};

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn createDoc(mut env: jni::JNIEnv) -> jobject {
    let doc = automerge::Automerge::new();
    doc.store_as_pointer(&mut env).unwrap().into_raw()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn createDocWithActor(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    actor_id_bytes: jni::sys::jbyteArray,
) -> jobject {
    let actor = env
        .convert_byte_array(JPrimitiveArray::from_raw(actor_id_bytes))
        .unwrap();
    let doc = automerge::Automerge::new().with_actor(actor.into());
    doc.store_as_pointer(&mut env).unwrap().into_raw()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getActorId(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
) -> jobject {
    let mut env_for_doc = env.unsafe_clone();
    let doc = automerge::Automerge::borrow_from_pointer(&mut env_for_doc, doc_pointer).unwrap();
    env.byte_array_from_slice(doc.get_actor().to_bytes())
        .unwrap()
        .into_raw()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn startTransaction(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jni::sys::jobject,
) -> jobject {
    let mut env_for_doc = env.unsafe_clone();
    let doc = automerge::Automerge::take_from_pointer(&mut env_for_doc, doc_pointer).unwrap();
    let tx = doc.into_transaction(None, None);
    tx.store_as_pointer(&mut env).unwrap().into_raw()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn startTransactionLogPatches(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jni::sys::jobject,
    patch_log_pointer: jni::sys::jobject,
) -> jobject {
    let doc = automerge::Automerge::take_from_pointer(&mut env, doc_pointer).unwrap();
    let patch_log = PatchLog::take_from_pointer(&mut env, patch_log_pointer).unwrap();
    let tx = doc.into_transaction(Some(patch_log), None);
    tx.store_as_pointer(&mut env).unwrap().into_raw()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn startTransactionAt(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jni::sys::jobject,
    patchlog_pointer: jni::sys::jobject,
    heads: jni::sys::jobject,
) -> jobject {
    let doc = automerge::Automerge::take_from_pointer(&mut env, doc_pointer).unwrap();
    let patch_log = PatchLog::take_from_pointer(&mut env, patchlog_pointer).unwrap();
    let heads = heads_from_jobject(&mut env, heads).unwrap();
    let tx = doc.into_transaction(Some(patch_log), Some(&heads));
    tx.store_as_pointer(&mut env).unwrap().into_raw()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn loadDoc(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    bytes_pointer: jbyteArray,
) -> jobject {
    let bytes = env
        .convert_byte_array(JPrimitiveArray::from_raw(bytes_pointer))
        .unwrap();
    let doc = match automerge::Automerge::load(&bytes) {
        Ok(d) => d,
        Err(e) => {
            throw_amg_exc_or_fatal(&mut env, e.to_string());
            return JObject::null().into_raw();
        }
    };
    doc.store_as_pointer(&mut env).unwrap().into_raw()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn freeDoc(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
) {
    let _doc = Automerge::take_from_pointer(&mut env, doc_pointer).unwrap();
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn saveDoc(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
) -> jobject {
    let doc = automerge::Automerge::take_from_pointer(&mut env, doc_pointer).unwrap();
    let bytes = doc.save();
    env.byte_array_from_slice(&bytes).unwrap().into_raw()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn forkDoc(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
) -> jobject {
    let mut env_for_doc = env.unsafe_clone();
    let doc = automerge::Automerge::borrow_from_pointer(&mut env_for_doc, doc_pointer).unwrap();
    doc.fork().store_as_pointer(&mut env).unwrap().into_raw()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn forkDocWithActor(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
    actor_bytes: jbyteArray,
) -> jobject {
    let actor_bytes = JPrimitiveArray::from_raw(actor_bytes);
    let actor = ActorId::from(env.convert_byte_array(&actor_bytes).unwrap());
    let mut env_for_doc = env.unsafe_clone();
    let doc = automerge::Automerge::borrow_from_pointer(&mut env_for_doc, doc_pointer).unwrap();
    doc.fork()
        .with_actor(actor)
        .store_as_pointer(&mut env)
        .unwrap()
        .into_raw()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn forkDocAt(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
    heads_pointer: jobject,
) -> jobject {
    do_fork_at(env, doc_pointer, heads_pointer, None)
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn forkDocAtWithActor(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
    heads_pointer: jobject,
    actor_bytes: jbyteArray,
) -> jobject {
    let actor_bytes = JPrimitiveArray::from_raw(actor_bytes);
    let actor = ActorId::from(env.convert_byte_array(&actor_bytes).unwrap());
    do_fork_at(env, doc_pointer, heads_pointer, Some(actor))
}

pub unsafe fn do_fork_at(
    mut env: jni::JNIEnv,
    doc_pointer: jobject,
    heads_pointer: jobject,
    new_actor: Option<ActorId>,
) -> jobject {
    let heads = heads_from_jobject(&mut env, heads_pointer).unwrap();
    let mut env_for_doc = env.unsafe_clone();
    let doc = automerge::Automerge::borrow_from_pointer(&mut env_for_doc, doc_pointer).unwrap();
    let doc = match doc.fork_at(&heads) {
        Ok(d) => d,
        Err(e) => {
            throw_amg_exc_or_fatal(&mut env, e.to_string());
            return JObject::null().into_raw();
        }
    };
    let doc = if let Some(new_actor) = new_actor {
        doc.with_actor(new_actor)
    } else {
        doc
    };
    doc.store_as_pointer(&mut env).unwrap().into_raw()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn mergeDoc(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
    other_doc_pointer: jobject,
) {
    let mut env_for_doc1 = env.unsafe_clone();
    let mut doc1 =
        automerge::Automerge::borrow_from_pointer(&mut env_for_doc1, doc_pointer).unwrap();
    let mut env_for_other_doc = env.unsafe_clone();
    let mut other_doc =
        automerge::Automerge::borrow_from_pointer(&mut env_for_other_doc, other_doc_pointer)
            .unwrap();
    match doc1.merge(&mut *other_doc) {
        Ok(_) => {}
        Err(e) => {
            throw_amg_exc_or_fatal(&mut env, e.to_string());
        }
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn mergeDocLogPatches(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
    other_doc_pointer: jobject,
    patch_log_pointer: jobject,
) {
    let mut env_for_doc1 = env.unsafe_clone();
    let mut doc1 =
        automerge::Automerge::borrow_from_pointer(&mut env_for_doc1, doc_pointer).unwrap();
    let mut env_for_other_doc = env.unsafe_clone();
    let mut other_doc =
        automerge::Automerge::borrow_from_pointer(&mut env_for_other_doc, other_doc_pointer)
            .unwrap();
    let mut env_for_patch_log = env.unsafe_clone();
    let mut patch_log =
        automerge::PatchLog::borrow_from_pointer(&mut env_for_patch_log, patch_log_pointer)
            .unwrap();
    match doc1.merge_and_log_patches(&mut *other_doc, &mut *patch_log) {
        Ok(_) => {}
        Err(e) => {
            throw_amg_exc_or_fatal(&mut env, e.to_string());
        }
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn encodeChangesSince(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
    heads_pointer: jobject,
) -> jobject {
    let mut env_for_doc = env.unsafe_clone();
    let doc = automerge::Automerge::borrow_from_pointer(&mut env_for_doc, doc_pointer).unwrap();
    let heads = heads_from_jobject(&mut env, heads_pointer).unwrap();
    let mut bytes = Vec::new();
    for change in doc.get_changes(&heads) {
        bytes.extend(change.raw_bytes().as_ref());
    }
    env.byte_array_from_slice(&bytes).unwrap().into_raw()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn applyEncodedChanges(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
    changes_pointer: jbyteArray,
) {
    let mut env_for_doc = env.unsafe_clone();
    let mut doc = automerge::Automerge::borrow_from_pointer(&mut env_for_doc, doc_pointer).unwrap();
    let changes_pointer = JPrimitiveArray::from_raw(changes_pointer);
    let changes_bytes = env.convert_byte_array(&changes_pointer).unwrap();
    match doc.load_incremental(&changes_bytes) {
        Ok(_) => {}
        Err(e) => {
            throw_amg_exc_or_fatal(&mut env, e.to_string());
        }
    };
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn applyEncodedChangesLogPatches(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
    patchlog_pointer: jobject,
    changes_pointer: jbyteArray,
) {
    let mut env_for_doc = env.unsafe_clone();
    let mut doc = automerge::Automerge::borrow_from_pointer(&mut env_for_doc, doc_pointer).unwrap();
    let changes_pointer = JPrimitiveArray::from_raw(changes_pointer);
    let changes_bytes = env.convert_byte_array(&changes_pointer).unwrap();
    let mut env_for_patchlog = env.unsafe_clone();
    let mut patchlog =
        PatchLog::borrow_from_pointer(&mut env_for_patchlog, patchlog_pointer).unwrap();
    match doc.load_incremental_log_patches(&changes_bytes, &mut *patchlog) {
        Ok(_) => {}
        Err(e) => {
            throw_amg_exc_or_fatal(&mut env, e.to_string());
        }
    };
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn makePatches(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
    patchlog_pointer: jobject,
) -> jobject {
    let mut env_for_doc = env.unsafe_clone();
    let doc = automerge::Automerge::borrow_from_pointer(&mut env_for_doc, doc_pointer).unwrap();
    let mut env_for_patchlog = env.unsafe_clone();
    let mut patchlog =
        PatchLog::borrow_from_pointer(&mut env_for_patchlog, patchlog_pointer).unwrap();
    let patches = doc.make_patches(&mut *patchlog);
    to_patch_arraylist(&mut env, patches).unwrap().into_raw()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn diff(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
    before_heads_pointer: jobject,
    after_heads_pointer: jobject,
) -> jobject {
    let mut env_for_doc = env.unsafe_clone();
    let doc = automerge::Automerge::borrow_from_pointer(&mut env_for_doc, doc_pointer).unwrap();
    let before = heads_from_jobject(&mut env, before_heads_pointer).unwrap();
    let after = heads_from_jobject(&mut env, after_heads_pointer).unwrap();
    let patches = doc.diff(&before, &after);
    to_patch_arraylist(&mut env, patches).unwrap().into_raw()
}
