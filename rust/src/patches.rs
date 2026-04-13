use automerge as am;
use jni::{jni_sig, jni_str, objects::JObject, signature::RuntimeMethodSignature, strings::JNIStr};

use crate::{
    am_value::to_amvalue,
    mark::mark_to_java,
    obj_id::JavaObjId,
    path_element::{path_to_java, prop_to_java, PATH_ELEM_CLASS, PROP_CLASS},
};

const PATCH_CLASS: &JNIStr = am_classname!("Patch");
const ACTION_CLASS: &JNIStr = am_classname!("PatchAction");
const PUTMAP_CLASS: &JNIStr = am_classname!("PatchAction$PutMap");
const PUTLIST_CLASS: &JNIStr = am_classname!("PatchAction$PutList");
const INSERT_CLASS: &JNIStr = am_classname!("PatchAction$Insert");
const SPLICE_TEXT_CLASS: &JNIStr = am_classname!("PatchAction$SpliceText");
const INCREMENT_CLASS: &JNIStr = am_classname!("PatchAction$Increment");
const DELETEMAP_CLASS: &JNIStr = am_classname!("PatchAction$DeleteMap");
const DELETELIST_CLASS: &JNIStr = am_classname!("PatchAction$DeleteList");
const MARK_CLASS: &JNIStr = am_classname!("PatchAction$Mark");
const FLAG_CONFLICT_CLASS: &JNIStr = am_classname!("PatchAction$FlagConflict");

fn to_jni_patch<'local>(
    env: &mut jni::Env<'local>,
    patch: am::Patch,
) -> jni::errors::Result<Option<JObject<'local>>> {
    let jaction = match patch.action {
        am::PatchAction::PutMap {
            key,
            value,
            conflict,
        } => {
            let jkey = env.new_string(key)?;
            let jvalue = unsafe { to_amvalue(env, value)? };

            // TODO: use jni_sig
            let constructor_sig = RuntimeMethodSignature::from_str(format!(
                "(Ljava/lang/String;L{};Z)V",
                am_classname!("AmValue")
            ))?;

            env.new_object(
                PUTMAP_CLASS,
                constructor_sig.method_signature(),
                &[(&jkey).into(), (&jvalue).into(), conflict.into()],
            )?
        }
        am::PatchAction::PutSeq {
            index,
            value,
            conflict,
        } => {
            let jvalue = unsafe { to_amvalue(env, value)? };
            let jindex = index as i64;

            // TODO: use jni_sig
            let constructor_sig =
                RuntimeMethodSignature::from_str(format!("(JL{};Z)V", am_classname!("AmValue")))
                    .unwrap();

            env.new_object(
                PUTLIST_CLASS,
                constructor_sig.method_signature(),
                &[jindex.into(), (&jvalue).into(), conflict.into()],
            )?
        }
        am::PatchAction::Insert { index, values } => {
            let arr = env.new_object_array(
                values.len() as i32,
                am_classname!("AmValue"),
                JObject::null(),
            )?;
            for (i, (value, id, _conflict)) in values.into_iter().enumerate() {
                let jval = unsafe { to_amvalue(env, (value.clone(), id.clone()))? };
                arr.set_element(env, i, jval)?;
            }
            let jindex = index as i64;

            let constructor_sig =
                RuntimeMethodSignature::from_str(format!("(J[L{};)V", am_classname!("AmValue")))
                    .unwrap();

            env.new_object(
                INSERT_CLASS,
                constructor_sig.method_signature(),
                &[jindex.into(), (&arr).into()],
            )?
        }
        am::PatchAction::SpliceText {
            index,
            value,
            marks: _,
        } => {
            let as_str = value.make_string();
            let jvalue = env.new_string(as_str)?;
            let jindex = index as i64;
            env.new_object(
                SPLICE_TEXT_CLASS,
                jni_sig!("(JLjava/lang/String;)V"),
                &[jindex.into(), (&jvalue).into()],
            )?
        }
        am::PatchAction::Increment { prop, value } => {
            let jprop = prop_to_java(env, &prop)?;

            // TODO: use jni_sig
            let constructor_sig = RuntimeMethodSignature::from_str(format!(
                "(L{PROP_CLASS};J)V",
                PROP_CLASS = am_classname!("Prop")
            ))?;
            env.new_object(
                INCREMENT_CLASS,
                constructor_sig.method_signature(),
                &[(&jprop).into(), value.into()],
            )?
        }
        am::PatchAction::DeleteMap { key } => {
            let jkey = env.new_string(key)?;
            env.new_object(
                DELETEMAP_CLASS,
                jni_sig!("(Ljava/lang/String;)V"),
                &[(&jkey).into()],
            )?
        }
        am::PatchAction::DeleteSeq { index, length } => {
            let jindex = index as i64;
            let jlength = length as i64;
            env.new_object(
                DELETELIST_CLASS,
                jni_sig!("(JJ)V"),
                &[jindex.into(), jlength.into()],
            )?
        }
        am::PatchAction::Mark { marks } => {
            let marks_arr =
                env.new_object_array(marks.len() as i32, am_classname!("Mark"), JObject::null())?;
            for (i, mark) in marks.into_iter().enumerate() {
                let jmark = mark_to_java(env, &mark)?;
                marks_arr.set_element(env, i, jmark)?
            }

            // TODO: use jni_sig
            let constructor_sig =
                RuntimeMethodSignature::from_str(format!("([L{};)V", am_classname!("Mark")))
                    .unwrap();
            env.new_object(
                MARK_CLASS,
                constructor_sig.method_signature(),
                &[(&marks_arr).into()],
            )?
        }
        am::PatchAction::Conflict { prop } => {
            let jprop = prop_to_java(env, &prop)?;

            // TODO: use jni_sig
            let constructor_sig =
                RuntimeMethodSignature::from_str(format!("(L{};)V", PROP_CLASS)).unwrap();

            env.new_object(
                FLAG_CONFLICT_CLASS,
                constructor_sig.method_signature(),
                &[(&jprop).into()],
            )?
        }
    };
    let jpath = path_to_java(env, patch.path.iter())?;
    let jid = JavaObjId::from(patch.obj).into_jobject(env)?;
    // TODO: use jni_sig
    let constructor_sig = RuntimeMethodSignature::from_str(format!(
        "(L{};[L{};L{};)V",
        am_classname!("ObjectId"),
        PATH_ELEM_CLASS,
        ACTION_CLASS
    ))?;
    env.new_object(
        PATCH_CLASS,
        constructor_sig.method_signature(),
        &[(&jid).into(), (&jpath).into(), (&jaction).into()],
    )
    .map(Some)
}

pub(crate) fn to_patch_arraylist<'local>(
    env: &mut jni::Env<'local>,
    patches: Vec<am::Patch>,
) -> jni::errors::Result<JObject<'local>> {
    let patches_arraylist =
        env.new_object(jni_str!("java/util/ArrayList"), jni_sig!("()V"), &[])?;
    for patch in patches.into_iter() {
        if let Some(jpatch) = to_jni_patch(env, patch)? {
            env.call_method(
                &patches_arraylist,
                jni_str!("add"),
                jni_sig!("(Ljava/lang/Object;)Z"),
                &[(&jpatch).into()],
            )?;
        }
    }
    Ok(patches_arraylist)
}
