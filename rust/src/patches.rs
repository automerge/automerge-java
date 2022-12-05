use automerge as am;
use jni::objects::JObject;

use crate::{
    am_value::to_amvalue,
    interop::changehash_to_jobject,
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

const HASH_AND_PATCH_CLASS: &str = am_classname!("HashAndPatches");

fn to_jni_patch<'a>(
    env: &jni::JNIEnv<'a>,
    patch: am::Patch<char>,
) -> jni::errors::Result<Option<JObject<'a>>> {
    let jaction = match patch.action {
        am::PatchAction::PutMap {
            key,
            value,
            expose: _,
            conflict,
        } => {
            let jkey = env.new_string(key).unwrap().into();
            let jvalue = unsafe { to_amvalue(env, value)? };

            env.new_object(
                PUTMAP_CLASS,
                format!("(Ljava/lang/String;L{};Z)V", am_classname!("AmValue")),
                &[jkey, jvalue.into(), conflict.into()],
            )?
        }
        am::PatchAction::PutSeq {
            index,
            value,
            expose: _,
            conflict,
        } => {
            let jvalue = unsafe { to_amvalue(env, value)? };
            let jindex = index as i64;
            env.new_object(
                PUTLIST_CLASS,
                format!("(JL{};Z)V", am_classname!("AmValue")),
                &[jindex.into(), jvalue.into(), conflict.into()],
            )?
        }
        am::PatchAction::Insert {
            index,
            values,
            conflict,
        } => {
            let arr = env.new_object_array(
                values.len() as i32,
                am_classname!("AmValue"),
                JObject::null(),
            )?;
            for (i, val) in values.into_iter().enumerate() {
                let jval = unsafe { to_amvalue(env, val.clone())? };
                env.set_object_array_element(arr, i as i32, jval)?;
            }
            let jarr = unsafe { JObject::from_raw(arr) };
            let jindex = index as i64;
            env.new_object(
                INSERT_CLASS,
                format!("(J[L{};Z)V", am_classname!("AmValue")),
                &[jindex.into(), jarr.into(), conflict.into()],
            )?
        }
        am::PatchAction::SpliceText { index, value } => {
            let as_str = value.into_iter().collect::<String>();
            let jvalue = env.new_string(as_str).unwrap();
            let jindex = index as i64;
            env.new_object(
                SPLICE_TEXT_CLASS,
                "(JLjava/lang/String;)V",
                &[jindex.into(), jvalue.into()],
            )?
        }
        am::PatchAction::Increment { prop, value } => {
            let jprop = prop_to_java(env, &prop)?;
            env.new_object(
                INCREMENT_CLASS,
                format!("(L{};J)V", PROP_CLASS),
                &[jprop.into(), value.into()],
            )?
        }
        am::PatchAction::DeleteMap { key } => {
            let jkey = env.new_string(key).unwrap().into();
            env.new_object(DELETEMAP_CLASS, "(Ljava/lang/String;)V", &[jkey])?
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
                env.set_object_array_element(marks_arr, i as i32, jmark)?;
            }
            let marks_arr = unsafe { JObject::from_raw(marks_arr) };
            env.new_object(
                MARK_CLASS,
                format!("([L{};)V", am_classname!("Mark")),
                &[marks_arr.into()],
            )?
        }
        // This is only here until we update automerge to remove this patch type
        am::PatchAction::Unmark { .. } => {
            return Ok(None);
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
        &[jid.into(), jpath.into(), jaction.into()],
    )
    .map(Some)
}

pub(crate) fn to_patch_arraylist<'a>(
    env: &jni::JNIEnv<'a>,
    patches: Vec<am::Patch<char>>,
) -> jni::errors::Result<JObject<'a>> {
    let patches_arraylist = env.new_object("java/util/ArrayList", "()V", &[])?;
    for patch in patches.into_iter() {
        if let Some(jpatch) = to_jni_patch(env, patch).unwrap() {
            env.call_method(
                patches_arraylist,
                "add",
                "(Ljava/lang/Object;)Z",
                &[jpatch.into()],
            )?;
        }
    }
    Ok(patches_arraylist)
}

pub(crate) fn hash_and_patches<'a>(
    env: &jni::JNIEnv<'a>,
    hash: &am::ChangeHash,
    patches: Vec<am::Patch<char>>,
) -> jni::errors::Result<JObject<'a>> {
    let jpatches = patches
        .into_iter()
        .filter_map(|patch| to_jni_patch(env, patch).transpose())
        .collect::<jni::errors::Result<Vec<_>>>()?;
    let patches_arr = env.new_object_array(jpatches.len() as i32, PATCH_CLASS, JObject::null())?;
    for (i, jpatch) in jpatches.into_iter().enumerate() {
        env.set_object_array_element(patches_arr, i as i32, jpatch)?;
    }
    let hash = unsafe { changehash_to_jobject(env, hash)? };
    let patches_arr = unsafe { JObject::from_raw(patches_arr) };
    env.new_object(
        HASH_AND_PATCH_CLASS,
        format!("([L{};L{};)V", PATCH_CLASS, am_classname!("ChangeHash")),
        &[patches_arr.into(), hash.into()],
    )
}
