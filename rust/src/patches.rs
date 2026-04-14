use automerge as am;
use jni::objects::{JObject, JObjectArray};

use crate::{
    am_value::to_amvalue,
    bindings::{
        AmValue, ArrayList, DeleteList, DeleteMap, FlagConflict, Increment, Mark, Patch,
        PatchAction, PatchActionInsert, PatchActionMark, PutList, PutMap, SpliceText,
    },
    mark::mark_to_java,
    obj_id::JavaObjId,
    path_element::{path_to_java, prop_to_java},
};

fn to_jni_patch<'local>(
    env: &mut jni::Env<'local>,
    patch: am::Patch,
) -> jni::errors::Result<Option<Patch<'local>>> {
    let jaction: PatchAction<'local> = match patch.action {
        am::PatchAction::PutMap {
            key,
            value,
            conflict,
        } => {
            let jkey = env.new_string(key)?;
            let jvalue = unsafe { to_amvalue(env, value)? };
            PutMap::new(env, &jkey, &jvalue, conflict)?.into()
        }
        am::PatchAction::PutSeq {
            index,
            value,
            conflict,
        } => {
            let jvalue = unsafe { to_amvalue(env, value)? };
            PutList::new(env, index as i64, &jvalue, conflict)?.into()
        }
        am::PatchAction::Insert { index, values } => {
            let arr = JObjectArray::<AmValue>::new(env, values.len(), &AmValue::null())?;
            for (i, (value, id, _conflict)) in values.into_iter().enumerate() {
                let jval = unsafe { to_amvalue(env, (value.clone(), id.clone()))? };
                arr.set_element(env, i, jval)?;
            }
            PatchActionInsert::new(env, index as i64, &arr)?.into()
        }
        am::PatchAction::SpliceText {
            index,
            value,
            marks: _,
        } => {
            let text = env.new_string(value.make_string())?;
            SpliceText::new(env, index as i64, &text)?.into()
        }
        am::PatchAction::Increment { prop, value } => {
            let jprop = prop_to_java(env, &prop)?;
            Increment::new(env, &jprop, value)?.into()
        }
        am::PatchAction::DeleteMap { key } => {
            let jkey = env.new_string(key)?;
            DeleteMap::new(env, &jkey)?.into()
        }
        am::PatchAction::DeleteSeq { index, length } => {
            DeleteList::new(env, index as i64, length as i64)?.into()
        }
        am::PatchAction::Mark { marks } => {
            let marks_arr = JObjectArray::<Mark>::new(env, marks.len(), &Mark::null())?;
            for (i, mark) in marks.into_iter().enumerate() {
                let jmark = mark_to_java(env, &mark)?;
                marks_arr.set_element(env, i, jmark)?;
            }
            PatchActionMark::new(env, &marks_arr)?.into()
        }
        am::PatchAction::Conflict { prop } => {
            let jprop = prop_to_java(env, &prop)?;
            FlagConflict::new(env, &jprop)?.into()
        }
    };
    let jpath = path_to_java(env, patch.path.iter())?;
    let jid = JavaObjId::from(patch.obj).into_object_id(env)?;
    Patch::new(env, &jid, &jpath, &jaction).map(Some)
}

pub(crate) fn to_patch_arraylist<'local>(
    env: &mut jni::Env<'local>,
    patches: Vec<am::Patch>,
) -> jni::errors::Result<JObject<'local>> {
    let list = ArrayList::new(env)?;
    for patch in patches {
        if let Some(jpatch) = to_jni_patch(env, patch)? {
            let jpatch_obj: JObject = jpatch.into();
            list.add(env, &jpatch_obj)?;
        }
    }
    Ok(list.into())
}
