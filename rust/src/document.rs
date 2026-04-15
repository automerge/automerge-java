use automerge::{ActorId, Automerge, AutomergeError, PatchLog};
use jni::{
    objects::{JByteArray, JClass, JObject, JObjectArray},
    NativeMethod,
};

use crate::{
    bindings,
    interop::{heads_from_jobject, unwrap_or_throw_amg_exc, JavaPointer},
    patches::to_patch_arraylist,
};

const _METHODS: &[NativeMethod] = &[
    ams_native! { static extern fn create_doc() -> bindings::DocPointer },
    ams_native! { static extern fn create_doc_with_actor(actor_id: jbyte[]) -> bindings::DocPointer },
    ams_native! { static extern fn load_doc(bytes: jbyte[]) -> bindings::DocPointer },
    ams_native! { static extern fn free_doc(doc: bindings::DocPointer) },
    ams_native! { static extern fn save_doc(doc: bindings::DocPointer) -> jbyte[] },
    ams_native! { static extern fn fork_doc(doc: bindings::DocPointer) -> bindings::DocPointer },
    ams_native! { static extern fn fork_doc_with_actor(doc: bindings::DocPointer, actor_bytes: jbyte[]) -> bindings::DocPointer },
    ams_native! { static extern fn fork_doc_at(doc: bindings::DocPointer, heads: bindings::ChangeHash[]) -> bindings::DocPointer },
    ams_native! { static extern fn fork_doc_at_with_actor(doc: bindings::DocPointer, heads: bindings::ChangeHash[], actor_bytes: jbyte[]) -> bindings::DocPointer },
    ams_native! { static extern fn merge_doc(doc: bindings::DocPointer, other: bindings::DocPointer) },
    ams_native! { static extern fn merge_doc_log_patches(doc: bindings::DocPointer, other: bindings::DocPointer, patch_log: bindings::PatchLogPointer) },
    ams_native! { static extern fn get_actor_id(doc: bindings::DocPointer) -> jbyte[] },
    ams_native! { static extern fn start_transaction(doc: bindings::DocPointer) -> bindings::TransactionPointer },
    ams_native! { static extern fn start_transaction_log_patches(doc: bindings::DocPointer, patch_log: bindings::PatchLogPointer) -> bindings::TransactionPointer },
    ams_native! { static extern fn start_transaction_at(doc: bindings::DocPointer, patch_log: bindings::PatchLogPointer, heads: bindings::ChangeHash[]) -> bindings::TransactionPointer },
    ams_native! { static extern fn encode_changes_since(doc: bindings::DocPointer, heads: bindings::ChangeHash[]) -> jbyte[] },
    ams_native! { static extern fn apply_encoded_changes(doc: bindings::DocPointer, changes: jbyte[]) },
    ams_native! { static extern fn apply_encoded_changes_log_patches(doc: bindings::DocPointer, patch_log: bindings::PatchLogPointer, changes: jbyte[]) },
    ams_native! { static extern fn make_patches(doc: bindings::DocPointer, patch_log: bindings::PatchLogPointer) -> JObject },
    ams_native! { static extern fn diff(doc: bindings::DocPointer, before_heads: bindings::ChangeHash[], after_heads: bindings::ChangeHash[]) -> JObject },
];

fn create_doc<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
) -> jni::errors::Result<bindings::DocPointer<'local>> {
    unsafe { Automerge::new().store_as_pointer(env) }
}

fn create_doc_with_actor<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    actor_id: JByteArray<'local>,
) -> jni::errors::Result<bindings::DocPointer<'local>> {
    let actor = env.convert_byte_array(&actor_id)?;
    unsafe {
        Automerge::new()
            .with_actor(actor.into())
            .store_as_pointer(env)
    }
}

fn load_doc<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    bytes: JByteArray<'local>,
) -> jni::errors::Result<bindings::DocPointer<'local>> {
    let bytes = env.convert_byte_array(&bytes)?;
    let doc = unwrap_or_throw_amg_exc::<_, AutomergeError>(env, Automerge::load(&bytes))?;
    unsafe { doc.store_as_pointer(env) }
}

fn free_doc<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
) -> jni::errors::Result<()> {
    let _doc = unsafe { Automerge::take_from_pointer(env, doc)? };
    Ok(())
}

fn save_doc<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
) -> jni::errors::Result<JByteArray<'local>> {
    let doc = unsafe { Automerge::borrow_from_pointer(env, doc)? };
    env.byte_array_from_slice(&doc.save())
}

fn fork_doc<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
) -> jni::errors::Result<bindings::DocPointer<'local>> {
    let doc = unsafe { Automerge::borrow_from_pointer(env, doc)? };
    unsafe { doc.fork().store_as_pointer(env) }
}

fn fork_doc_with_actor<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
    actor_bytes: JByteArray<'local>,
) -> jni::errors::Result<bindings::DocPointer<'local>> {
    let actor = ActorId::from(env.convert_byte_array(&actor_bytes)?);
    let doc = unsafe { Automerge::borrow_from_pointer(env, doc)? };
    unsafe { doc.fork().with_actor(actor).store_as_pointer(env) }
}

fn fork_doc_at<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
    heads: JObjectArray<'local, bindings::ChangeHash<'local>>,
) -> jni::errors::Result<bindings::DocPointer<'local>> {
    unsafe { do_fork_at(env, doc.into(), heads, None) }
}

fn fork_doc_at_with_actor<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
    heads: JObjectArray<'local, bindings::ChangeHash<'local>>,
    actor_bytes: JByteArray<'local>,
) -> jni::errors::Result<bindings::DocPointer<'local>> {
    let actor = ActorId::from(env.convert_byte_array(&actor_bytes)?);
    unsafe { do_fork_at(env, doc.into(), heads, Some(actor)) }
}

unsafe fn do_fork_at<'local>(
    env: &mut jni::Env<'local>,
    doc: JObject<'local>,
    heads: JObjectArray<'local, bindings::ChangeHash<'local>>,
    new_actor: Option<ActorId>,
) -> Result<bindings::DocPointer<'local>, jni::errors::Error> {
    let heads = heads_from_jobject(env, heads)?;
    let doc = Automerge::borrow_from_pointer(env, doc)?;
    let doc = unwrap_or_throw_amg_exc::<_, AutomergeError>(env, doc.fork_at(&heads))?;
    let doc = if let Some(new_actor) = new_actor {
        doc.with_actor(new_actor)
    } else {
        doc
    };
    doc.store_as_pointer(env)
}

fn merge_doc<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
    other: bindings::DocPointer<'local>,
) -> jni::errors::Result<()> {
    let mut doc1 = unsafe { Automerge::borrow_from_pointer(env, doc)? };
    let mut other_doc = unsafe { Automerge::borrow_from_pointer(env, other)? };
    unwrap_or_throw_amg_exc::<_, AutomergeError>(env, doc1.merge(&mut other_doc))?;
    Ok(())
}

fn merge_doc_log_patches<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
    other: bindings::DocPointer<'local>,
    patch_log: bindings::PatchLogPointer<'local>,
) -> jni::errors::Result<()> {
    let mut doc1 = unsafe { Automerge::borrow_from_pointer(env, doc)? };
    let mut other_doc = unsafe { Automerge::borrow_from_pointer(env, other)? };
    let mut patch_log = unsafe { PatchLog::borrow_from_pointer(env, patch_log)? };
    unwrap_or_throw_amg_exc::<_, AutomergeError>(
        env,
        doc1.merge_and_log_patches(&mut other_doc, &mut patch_log),
    )?;
    Ok(())
}

fn get_actor_id<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
) -> jni::errors::Result<JByteArray<'local>> {
    let doc = unsafe { Automerge::borrow_from_pointer(env, doc)? };
    env.byte_array_from_slice(doc.get_actor().to_bytes())
}

fn start_transaction<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
) -> jni::errors::Result<bindings::TransactionPointer<'local>> {
    let doc = unsafe { Automerge::take_from_pointer(env, doc)? };
    let tx = doc.into_transaction(None, None);
    unsafe { tx.store_as_pointer(env) }
}

fn start_transaction_log_patches<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
    patch_log: bindings::PatchLogPointer<'local>,
) -> jni::errors::Result<bindings::TransactionPointer<'local>> {
    let doc = unsafe { Automerge::take_from_pointer(env, doc)? };
    let patch_log = unsafe { PatchLog::take_from_pointer(env, patch_log)? };
    let tx = doc.into_transaction(Some(patch_log), None);
    unsafe { tx.store_as_pointer(env) }
}

fn start_transaction_at<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
    patch_log: bindings::PatchLogPointer<'local>,
    heads: JObjectArray<'local, bindings::ChangeHash<'local>>,
) -> jni::errors::Result<bindings::TransactionPointer<'local>> {
    let doc = unsafe { Automerge::take_from_pointer(env, doc)? };
    let patch_log = unsafe { PatchLog::take_from_pointer(env, patch_log)? };
    let heads = heads_from_jobject(env, heads)?;
    let tx = doc.into_transaction(Some(patch_log), Some(&heads));
    unsafe { tx.store_as_pointer(env) }
}

fn encode_changes_since<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
    heads: JObjectArray<'local, bindings::ChangeHash<'local>>,
) -> jni::errors::Result<JByteArray<'local>> {
    let doc = unsafe { Automerge::borrow_from_pointer(env, doc)? };
    let heads = heads_from_jobject(env, heads)?;
    let mut bytes = Vec::new();
    for change in doc.get_changes(&heads) {
        bytes.extend(change.raw_bytes().as_ref());
    }
    env.byte_array_from_slice(&bytes)
}

fn apply_encoded_changes<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
    changes: JByteArray<'local>,
) -> jni::errors::Result<()> {
    let mut doc = unsafe { Automerge::borrow_from_pointer(env, doc)? };
    let bytes = env.convert_byte_array(&changes)?;
    unwrap_or_throw_amg_exc::<_, AutomergeError>(env, doc.load_incremental(&bytes))?;
    Ok(())
}

fn apply_encoded_changes_log_patches<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
    patch_log: bindings::PatchLogPointer<'local>,
    changes: JByteArray<'local>,
) -> jni::errors::Result<()> {
    let mut doc = unsafe { Automerge::borrow_from_pointer(env, doc)? };
    let bytes = env.convert_byte_array(&changes)?;
    let mut patch_log = unsafe { PatchLog::borrow_from_pointer(env, patch_log)? };
    unwrap_or_throw_amg_exc::<_, AutomergeError>(
        env,
        doc.load_incremental_log_patches(&bytes, &mut patch_log),
    )?;
    Ok(())
}

fn make_patches<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
    patch_log: bindings::PatchLogPointer<'local>,
) -> jni::errors::Result<JObject<'local>> {
    let doc = unsafe { Automerge::borrow_from_pointer(env, doc)? };
    let mut patch_log = unsafe { PatchLog::borrow_from_pointer(env, patch_log)? };
    to_patch_arraylist(env, doc.make_patches(&mut patch_log))
}

fn diff<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
    before_heads: JObjectArray<'local, bindings::ChangeHash<'local>>,
    after_heads: JObjectArray<'local, bindings::ChangeHash<'local>>,
) -> jni::errors::Result<JObject<'local>> {
    let doc = unsafe { Automerge::borrow_from_pointer(env, doc)? };
    let before = heads_from_jobject(env, before_heads)?;
    let after = heads_from_jobject(env, after_heads)?;
    to_patch_arraylist(env, doc.diff(&before, &after))
}
