use automerge::{ActorId, Automerge, AutomergeError, PatchLog};
use automerge_jni_macros::jni_fn;
use jni::{
    errors::ThrowRuntimeExAndDefault,
    objects::{JByteArray, JClass, JObject, JObjectArray},
};

use crate::{
    interop::{heads_from_jobject, unwrap_or_throw_amg_exc, JavaPointer},
    patches::to_patch_arraylist,
};

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn createDoc<'local>(mut env: jni::EnvUnowned<'local>) -> JObject<'local> {
    env.with_env(|env| {
        let doc = automerge::Automerge::new();
        doc.store_as_pointer(env)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn createDocWithActor<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    actor_id_bytes: JByteArray<'local>,
) -> JObject<'local> {
    env.with_env(|env| {
        let actor = env.convert_byte_array(actor_id_bytes)?;
        let doc = automerge::Automerge::new().with_actor(actor.into());
        doc.store_as_pointer(env)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn getActorId<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc_pointer: JObject<'local>,
) -> JObject<'local> {
    env.with_env(|env| {
        let doc = automerge::Automerge::borrow_from_pointer(env, doc_pointer)?;
        Ok::<_, jni::errors::Error>(
            env.byte_array_from_slice(doc.get_actor().to_bytes())?
                .into(),
        )
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn startTransaction<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc_pointer: JObject<'local>,
) -> JObject<'local> {
    env.with_env(|env| {
        let doc = automerge::Automerge::take_from_pointer(env, doc_pointer)?;
        let tx = doc.into_transaction(None, None);
        tx.store_as_pointer(env)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn startTransactionLogPatches<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    patch_log: JObject<'local>,
) -> JObject<'local> {
    env.with_env(|env| {
        let doc = automerge::Automerge::take_from_pointer(env, doc)?;
        let patch_log = PatchLog::take_from_pointer(env, patch_log)?;
        let tx = doc.into_transaction(Some(patch_log), None);
        tx.store_as_pointer(env)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn startTransactionAt<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    patchlog: JObject<'local>,
    heads: JObjectArray<'local>,
) -> JObject<'local> {
    env.with_env(|env| {
        let doc = automerge::Automerge::take_from_pointer(env, doc)?;
        let patch_log = PatchLog::take_from_pointer(env, patchlog)?;
        let heads = heads_from_jobject(env, heads)?;
        let tx = doc.into_transaction(Some(patch_log), Some(&heads));
        tx.store_as_pointer(env)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn loadDoc<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    bytes: JByteArray<'local>,
) -> JObject<'local> {
    env.with_env(|env| {
        let bytes = env.convert_byte_array(bytes)?;
        let doc =
            unwrap_or_throw_amg_exc::<_, AutomergeError>(env, automerge::Automerge::load(&bytes))?;
        doc.store_as_pointer(env)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn freeDoc<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
) {
    env.with_env(|env| {
        let _doc = Automerge::take_from_pointer(env, doc)?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>();
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn saveDoc<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
) -> JByteArray<'local> {
    env.with_env(|env| {
        let doc = automerge::Automerge::take_from_pointer(env, doc)?;
        let bytes = doc.save();
        env.byte_array_from_slice(&bytes)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn forkDoc<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
) -> JObject<'local> {
    env.with_env(|env| {
        let doc = automerge::Automerge::borrow_from_pointer(env, doc)?;
        doc.fork().store_as_pointer(env)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn forkDocWithActor<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    actor_bytes: JByteArray<'local>,
) -> JObject<'local> {
    env.with_env(|env| {
        let actor = ActorId::from(env.convert_byte_array(&actor_bytes)?);
        let doc = automerge::Automerge::borrow_from_pointer(env, doc)?;
        doc.fork().with_actor(actor).store_as_pointer(env)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn forkDocAt<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    heads: JObjectArray<'local>,
) -> JObject<'local> {
    env.with_env(|env| do_fork_at(env, doc, heads, None))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn forkDocAtWithActor<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    heads: JObjectArray<'local>,
    actor_bytes: JByteArray<'local>,
) -> JObject<'local> {
    env.with_env(|env| {
        let actor = ActorId::from(env.convert_byte_array(&actor_bytes)?);
        do_fork_at(env, doc, heads, Some(actor))
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

pub unsafe fn do_fork_at<'local>(
    env: &mut jni::Env<'local>,
    doc: JObject<'local>,
    heads: JObjectArray<'local>,
    new_actor: Option<ActorId>,
) -> Result<JObject<'local>, jni::errors::Error> {
    let heads = heads_from_jobject(env, heads)?;
    let doc = automerge::Automerge::borrow_from_pointer(env, doc)?;
    let doc = unwrap_or_throw_amg_exc::<_, AutomergeError>(env, doc.fork_at(&heads))?;
    let doc = if let Some(new_actor) = new_actor {
        doc.with_actor(new_actor)
    } else {
        doc
    };
    doc.store_as_pointer(env)
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn mergeDoc<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc_pointer: JObject<'local>,
    other_doc: JObject<'local>,
) {
    env.with_env(|env| {
        let mut doc1 = automerge::Automerge::borrow_from_pointer(env, doc_pointer)?;
        let mut other_doc = automerge::Automerge::borrow_from_pointer(env, other_doc)?;
        unwrap_or_throw_amg_exc::<_, AutomergeError>(env, doc1.merge(&mut other_doc))?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn mergeDocLogPatches<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    other_doc: JObject<'local>,
    patch_log: JObject<'local>,
) {
    env.with_env(|env| {
        let mut doc1 = automerge::Automerge::borrow_from_pointer(env, doc)?;
        let mut other_doc = automerge::Automerge::borrow_from_pointer(env, other_doc)?;
        let mut patch_log = automerge::PatchLog::borrow_from_pointer(env, patch_log)?;
        unwrap_or_throw_amg_exc::<_, AutomergeError>(
            env,
            doc1.merge_and_log_patches(&mut other_doc, &mut patch_log),
        )?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn encodeChangesSince<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    heads: JObjectArray<'local>,
) -> JByteArray<'local> {
    env.with_env(|env| {
        let doc = automerge::Automerge::borrow_from_pointer(env, doc)?;
        let heads = heads_from_jobject(env, heads)?;
        let mut bytes = Vec::new();
        for change in doc.get_changes(&heads) {
            bytes.extend(change.raw_bytes().as_ref());
        }
        env.byte_array_from_slice(&bytes)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn applyEncodedChanges<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    changes: JByteArray<'local>,
) {
    env.with_env(|env| {
        let mut doc = automerge::Automerge::borrow_from_pointer(env, doc)?;
        let changes_bytes = env.convert_byte_array(&changes)?;
        unwrap_or_throw_amg_exc::<_, AutomergeError>(env, doc.load_incremental(&changes_bytes))?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn applyEncodedChangesLogPatches<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    patchlog: JObject<'local>,
    changes: JByteArray<'local>,
) {
    env.with_env(|env| {
        let mut doc = automerge::Automerge::borrow_from_pointer(env, doc)?;
        let changes_bytes = env.convert_byte_array(&changes)?;
        let mut patchlog = PatchLog::borrow_from_pointer(env, patchlog)?;
        unwrap_or_throw_amg_exc::<_, AutomergeError>(
            env,
            doc.load_incremental_log_patches(&changes_bytes, &mut patchlog),
        )?;
        Ok::<_, jni::errors::Error>(())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn makePatches<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    patchlog: JObject<'local>,
) -> JObject<'local> {
    env.with_env(|env| {
        let doc = automerge::Automerge::borrow_from_pointer(env, doc)?;
        let mut patchlog = PatchLog::borrow_from_pointer(env, patchlog)?;
        let patches = doc.make_patches(&mut patchlog);
        to_patch_arraylist(env, patches)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn diff<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    doc: JObject<'local>,
    before_heads: JObjectArray<'local>,
    after_heads: JObjectArray<'local>,
) -> JObject<'local> {
    env.with_env(|env| {
        let doc = automerge::Automerge::borrow_from_pointer(env, doc)?;
        let before = heads_from_jobject(env, before_heads)?;
        let after = heads_from_jobject(env, after_heads)?;
        let patches = doc.diff(&before, &after);
        to_patch_arraylist(env, patches)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}
