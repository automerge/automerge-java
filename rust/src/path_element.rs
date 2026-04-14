use automerge as am;
use jni::objects::JObjectArray;

use crate::{
    bindings::{PathElement, Prop, PropIndex, PropKey},
    obj_id::JavaObjId,
};

pub(crate) fn prop_to_java<'local>(
    env: &mut jni::Env<'local>,
    prop: &am::Prop,
) -> jni::errors::Result<Prop<'local>> {
    match prop {
        am::Prop::Map(key) => {
            let jkey = env.new_string(key)?;
            Ok(PropKey::new(env, &jkey)?.into())
        }
        am::Prop::Seq(idx) => Ok(PropIndex::new(env, *idx as i64)?.into()),
    }
}

pub(crate) fn path_to_java<'local, 'b, I: Iterator<Item = &'b (am::ObjId, am::Prop)>>(
    env: &mut jni::Env<'local>,
    path: I,
) -> jni::errors::Result<JObjectArray<'local, PathElement<'local>>> {
    let mut elems: Vec<PathElement<'local>> = Vec::new();
    for (id, prop) in path {
        let jid = JavaObjId::from(id.clone()).into_object_id(env)?;
        let jprop = prop_to_java(env, prop)?;
        elems.push(PathElement::new(env, &jid, &jprop)?);
    }
    let arr = JObjectArray::<PathElement>::new(env, elems.len(), &PathElement::null())?;
    for (i, elem) in elems.into_iter().enumerate() {
        arr.set_element(env, i, elem)?;
    }
    Ok(arr)
}
