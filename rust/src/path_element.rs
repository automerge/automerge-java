use automerge as am;
use jni::objects::JObject;

use crate::obj_id::JavaObjId;

pub(crate) const PATH_ELEM_CLASS: &str = am_classname!("PathElement");
pub(crate) const PROP_CLASS: &str = am_classname!("Prop");
pub(crate) const KEY_CLASS: &str = am_classname!("Prop$Key");
pub(crate) const IDX_CLASS: &str = am_classname!("Prop$Index");

pub(crate) fn prop_to_java<'a>(
    env: &mut jni::JNIEnv<'a>,
    prop: &am::Prop,
) -> jni::errors::Result<JObject<'a>> {
    match prop {
        am::Prop::Map(key) => {
            let jkey = env.new_string(key).unwrap();
            env.new_object(KEY_CLASS, "(Ljava/lang/String;)V", &[(&jkey).into()])
        }
        am::Prop::Seq(idx) => {
            let jidx = *idx as i64;
            env.new_object(IDX_CLASS, "(J)V", &[jidx.into()])
        }
    }
}

pub(crate) fn path_to_java<'a, 'b, I: Iterator<Item = &'b (am::ObjId, am::Prop)>>(
    env: &mut jni::JNIEnv<'a>,
    path: I,
) -> jni::errors::Result<JObject<'a>> {
    let path_class = env.find_class(PATH_ELEM_CLASS)?;
    let mut elems = Vec::new();
    for (id, prop) in path {
        let jid = JavaObjId::from(id.clone()).into_jobject(env)?;
        let jprop = prop_to_java(env, prop)?;
        let jpath_elem = env.new_object(
            &path_class,
            format!("(L{};L{};)V", am_classname!("ObjectId"), PROP_CLASS),
            &[(&jid).into(), (&jprop).into()],
        )?;
        elems.push(jpath_elem);
    }
    let arr = env.new_object_array(elems.len() as i32, PATH_ELEM_CLASS, JObject::null())?;
    for (i, elem) in elems.into_iter().enumerate() {
        env.set_object_array_element(&arr, i as i32, elem)?;
    }
    Ok(arr.into())
}
