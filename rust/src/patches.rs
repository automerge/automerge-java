use automerge as am;
use jni::objects::JObject;

use crate::{
    am_value::to_amvalue,
    mark::mark_to_java,
    obj_id::JavaObjId,
    path_element::{path_to_java, prop_to_java, PATH_ELEM_CLASS, PROP_CLASS},
};

const PATCH_CLASS: &str = am_classname!("Patch");
const ACTION_CLASS: &str = am_classname!("PatchAction");
const PUTMAP_CLASS: &str = am_classname!("PatchAction$PutMap");
const PUTLIST_CLASS: &str = am_classname!("PatchAction$PutList");
const INSERT_CLASS: &str = am_classname!("PatchAction$Insert");
const SPLICE_TEXT_CLASS: &str = am_classname!("PatchAction$SpliceText");
const INCREMENT_CLASS: &str = am_classname!("PatchAction$Increment");
const DELETEMAP_CLASS: &str = am_classname!("PatchAction$DeleteMap");
const DELETELIST_CLASS: &str = am_classname!("PatchAction$DeleteList");
const MARK_CLASS: &str = am_classname!("PatchAction$Mark");
const FLAG_CONFLICT_CLASS: &str = am_classname!("PatchAction$FlagConflict");

fn to_jni_patch<'a>(
    env: &mut jni::JNIEnv<'a>,
    patch: am::Patch,
) -> jni::errors::Result<Option<JObject<'a>>> {
    let jaction = match patch.action {
        am::PatchAction::PutMap {
            key,
            value,
            conflict,
        } => {
            let jkey = env.new_string(key).unwrap();
            let jvalue = unsafe { to_amvalue(env, value)? };

            env.new_object(
                PUTMAP_CLASS,
                format!("(Ljava/lang/String;L{};Z)V", am_classname!("AmValue")),
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
            env.new_object(
                PUTLIST_CLASS,
                format!("(JL{};Z)V", am_classname!("AmValue")),
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
                env.set_object_array_element(&arr, i as i32, jval)?;
            }
            let jindex = index as i64;
            env.new_object(
                INSERT_CLASS,
                format!("(J[L{};)V", am_classname!("AmValue")),
                &[jindex.into(), (&arr).into()],
            )?
        }
        am::PatchAction::SpliceText {
            index,
            value,
            marks: _,
        } => {
            let as_str = value.make_string();
            let jvalue = env.new_string(as_str).unwrap();
            let jindex = index as i64;
            env.new_object(
                SPLICE_TEXT_CLASS,
                "(JLjava/lang/String;)V",
                &[jindex.into(), (&jvalue).into()],
            )?
        }
        am::PatchAction::Increment { prop, value } => {
            let jprop = prop_to_java(env, &prop)?;
            env.new_object(
                INCREMENT_CLASS,
                format!("(L{PROP_CLASS};J)V"),
                &[(&jprop).into(), value.into()],
            )?
        }
        am::PatchAction::DeleteMap { key } => {
            let jkey = env.new_string(key).unwrap();
            env.new_object(DELETEMAP_CLASS, "(Ljava/lang/String;)V", &[(&jkey).into()])?
        }
        am::PatchAction::DeleteSeq { index, length } => {
            let jindex = index as i64;
            let jlength = length as i64;
            env.new_object(DELETELIST_CLASS, "(JJ)V", &[jindex.into(), jlength.into()])?
        }
        am::PatchAction::Mark { marks } => {
            let marks_arr =
                env.new_object_array(marks.len() as i32, am_classname!("Mark"), JObject::null())?;
            for (i, mark) in marks.into_iter().enumerate() {
                let jmark = mark_to_java(env, &mark)?;
                env.set_object_array_element(&marks_arr, i as i32, jmark)?;
            }
            env.new_object(
                MARK_CLASS,
                format!("([L{};)V", am_classname!("Mark")),
                &[(&marks_arr).into()],
            )?
        }
        am::PatchAction::Conflict { prop } => {
            let jprop = prop_to_java(env, &prop)?;
            env.new_object(
                FLAG_CONFLICT_CLASS,
                format!("(L{PROP_CLASS};J)V"),
                &[(&jprop).into(), 0.into()],
            )?
        }
    };
    let jpath = path_to_java(env, patch.path.iter())?;
    let jid = JavaObjId::from(patch.obj).into_jobject(env)?;
    env.new_object(
        PATCH_CLASS,
        format!(
            "(L{};[L{};L{};)V",
            am_classname!("ObjectId"),
            PATH_ELEM_CLASS,
            ACTION_CLASS
        ),
        &[(&jid).into(), (&jpath).into(), (&jaction).into()],
    )
    .map(Some)
}

pub(crate) fn to_patch_arraylist<'a>(
    env: &mut jni::JNIEnv<'a>,
    patches: Vec<am::Patch>,
) -> jni::errors::Result<JObject<'a>> {
    let patches_arraylist = env.new_object("java/util/ArrayList", "()V", &[])?;
    for patch in patches.into_iter() {
        if let Some(jpatch) = to_jni_patch(env, patch).unwrap() {
            env.call_method(
                &patches_arraylist,
                "add",
                "(Ljava/lang/Object;)Z",
                &[(&jpatch).into()],
            )?;
        }
    }
    Ok(patches_arraylist)
}
