use automerge::{op_observer::HasPatches, ActorId, Automerge, VecOpObserver};
use automerge_jni_macros::jni_fn;
use jni::{objects::JObject, sys::jobject};

use crate::{
    interop::{heads_from_jobject, AsPointerObj},
    patches::to_patch_arraylist,
    AUTOMERGE_EXCEPTION,
};

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn createDoc(env: jni::JNIEnv) -> jobject {
    let doc = automerge::Automerge::new();
    doc.to_pointer_obj(&env).unwrap()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn createDocWithActor(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    actor_id_bytes: jobject,
) -> jobject {
    let actor = env.convert_byte_array(actor_id_bytes).unwrap();
    let doc = automerge::Automerge::new().with_actor(actor.into());
    doc.to_pointer_obj(&env).unwrap()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getActorId(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
) -> jobject {
    let doc = automerge::Automerge::from_pointer_obj(&env, doc_pointer).unwrap();
    env.byte_array_from_slice(doc.get_actor().to_bytes())
        .unwrap()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn startTransaction(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jni::sys::jobject,
) -> jobject {
    let doc = automerge::Automerge::from_pointer_obj(&env, doc_pointer).unwrap();
    let tx = doc.transaction();
    tx.to_pointer_obj(&env).unwrap()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn startObservedTransaction(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jni::sys::jobject,
) -> jobject {
    let doc = automerge::Automerge::from_pointer_obj(&env, doc_pointer).unwrap();
    let observer = automerge::VecOpObserver::default()
        .with_text_rep(automerge::op_observer::TextRepresentation::String);
    let tx = doc.transaction_with_observer(observer);
    tx.to_pointer_obj(&env).unwrap()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn loadDoc(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    bytes_pointer: jobject,
) -> jobject {
    let bytes = env.convert_byte_array(bytes_pointer).unwrap();
    let doc = match automerge::Automerge::load(&bytes) {
        Ok(d) => d,
        Err(e) => {
            env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
            return JObject::null().into_raw();
        }
    };
    doc.to_pointer_obj(&env).unwrap()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn freeDoc(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
) {
    let _doc = Automerge::owned_from_pointer_obj(&env, doc_pointer).unwrap();
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn saveDoc(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
) -> jobject {
    let doc = automerge::Automerge::from_pointer_obj(&env, doc_pointer).unwrap();
    let bytes = doc.save();
    env.byte_array_from_slice(&bytes).unwrap()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn forkDoc(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
) -> jobject {
    let doc = automerge::Automerge::from_pointer_obj(&env, doc_pointer).unwrap();
    doc.fork().to_pointer_obj(&env).unwrap()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn forkDocWithActor(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
    actor_bytes: jobject,
) -> jobject {
    let actor = ActorId::from(env.convert_byte_array(actor_bytes).unwrap());
    let doc = automerge::Automerge::from_pointer_obj(&env, doc_pointer).unwrap();
    doc.fork().with_actor(actor).to_pointer_obj(&env).unwrap()
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
    actor_bytes: jobject,
) -> jobject {
    let actor = ActorId::from(env.convert_byte_array(actor_bytes).unwrap());
    do_fork_at(env, doc_pointer, heads_pointer, Some(actor))
}

pub unsafe fn do_fork_at(
    env: jni::JNIEnv,
    doc_pointer: jobject,
    heads_pointer: jobject,
    new_actor: Option<ActorId>,
) -> jobject {
    let heads = heads_from_jobject(&env, heads_pointer).unwrap();
    let doc = automerge::Automerge::from_pointer_obj(&env, doc_pointer).unwrap();
    let doc = match doc.fork_at(&heads) {
        Ok(d) => d,
        Err(e) => {
            env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
            return JObject::null().into_raw();
        }
    };
    let doc = if let Some(new_actor) = new_actor {
        doc.with_actor(new_actor)
    } else {
        doc
    };
    doc.to_pointer_obj(&env).unwrap()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn mergeDoc(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
    other_doc_pointer: jobject,
) {
    let doc1 = automerge::Automerge::from_pointer_obj(&env, doc_pointer).unwrap();
    let other_doc = automerge::Automerge::from_pointer_obj(&env, other_doc_pointer).unwrap();
    match doc1.merge(other_doc) {
        Ok(_) => {}
        Err(e) => {
            env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
        }
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn mergeDocObserved(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
    other_doc_pointer: jobject,
) -> jobject {
    let doc1 = automerge::Automerge::from_pointer_obj(&env, doc_pointer).unwrap();
    let other_doc = automerge::Automerge::from_pointer_obj(&env, other_doc_pointer).unwrap();
    let mut obs =
        VecOpObserver::default().with_text_rep(automerge::op_observer::TextRepresentation::String);
    match doc1.merge_with(other_doc, Some(&mut obs)) {
        Ok(_) => {}
        Err(e) => {
            env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
        }
    }
    let patches = obs.take_patches();
    to_patch_arraylist(&env, patches).unwrap().into_raw()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn encodeChangesSince(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
    heads_pointer: jobject,
) -> jobject {
    let doc = automerge::Automerge::from_pointer_obj(&env, doc_pointer).unwrap();
    let heads = heads_from_jobject(&env, heads_pointer).unwrap();
    let changes = match doc.get_changes(&heads) {
        Ok(c) => c,
        Err(e) => {
            env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
            return JObject::null().into_raw();
        }
    };
    let mut bytes = Vec::new();
    for change in changes {
        bytes.extend(change.raw_bytes().as_ref());
    }
    env.byte_array_from_slice(&bytes).unwrap()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn applyEncodedChanges(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
    changes_pointer: jobject,
) {
    let doc = automerge::Automerge::from_pointer_obj(&env, doc_pointer).unwrap();
    let changes_bytes = env.convert_byte_array(changes_pointer).unwrap();
    match doc.load_incremental(&changes_bytes) {
        Ok(_) => {}
        Err(e) => {
            env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
        }
    };
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn applyEncodedChangesObserved(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    doc_pointer: jobject,
    changes_pointer: jobject,
) -> jobject {
    let doc = automerge::Automerge::from_pointer_obj(&env, doc_pointer).unwrap();
    let changes_bytes = env.convert_byte_array(changes_pointer).unwrap();
    let mut observer =
        VecOpObserver::default().with_text_rep(automerge::op_observer::TextRepresentation::String);
    match doc.load_incremental_with(&changes_bytes, Some(&mut observer)) {
        Ok(_) => {}
        Err(e) => {
            env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
            return JObject::null().into_raw();
        }
    };
    let patches = observer.take_patches();
    to_patch_arraylist(&env, patches).unwrap().into_raw()
}
