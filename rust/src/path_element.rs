use automerge as am;
use jni::{jni_sig, objects::JObject, signature::RuntimeMethodSignature, strings::JNIStr};

use crate::obj_id::JavaObjId;

pub(crate) const PATH_ELEM_CLASS: &JNIStr = am_classname!("PathElement");
pub(crate) const PROP_CLASS: &JNIStr = am_classname!("Prop");
pub(crate) const KEY_CLASS: &JNIStr = am_classname!("Prop$Key");
pub(crate) const IDX_CLASS: &JNIStr = am_classname!("Prop$Index");

pub(crate) fn prop_to_java<'local>(
    env: &mut jni::Env<'local>,
    prop: &am::Prop,
) -> jni::errors::Result<JObject<'local>> {
    match prop {
        am::Prop::Map(key) => {
            let jkey = env.new_string(key)?;
            env.new_object(
                KEY_CLASS,
                jni_sig!("(Ljava/lang/String;)V"),
                &[(&jkey).into()],
            )
        }
        am::Prop::Seq(idx) => {
            let jidx = *idx as i64;
            env.new_object(IDX_CLASS, jni_sig!("(J)V"), &[jidx.into()])
        }
    }
}

pub(crate) fn path_to_java<'local, 'b, I: Iterator<Item = &'b (am::ObjId, am::Prop)>>(
    env: &mut jni::Env<'local>,
    path: I,
) -> jni::errors::Result<JObject<'local>> {
    let path_class = env.find_class(PATH_ELEM_CLASS)?;
    let mut elems = Vec::new();

    // TODO: replace with jni_sig
    let constructor_sig = RuntimeMethodSignature::from_str(format!(
        "(L{};L{};)V",
        am_classname!("ObjectId"),
        PROP_CLASS
    ))?;
    for (id, prop) in path {
        let jid = JavaObjId::from(id.clone()).into_jobject(env)?;
        let jprop = prop_to_java(env, prop)?;
        let jpath_elem = env.new_object(
            &path_class,
            constructor_sig.method_signature(),
            &[(&jid).into(), (&jprop).into()],
        )?;
        elems.push(jpath_elem);
    }
    let arr = env.new_object_array(elems.len() as i32, PATH_ELEM_CLASS, JObject::null())?;
    for (i, elem) in elems.into_iter().enumerate() {
        arr.set_element(env, i, elem)?;
    }
    Ok(arr.into())
}
