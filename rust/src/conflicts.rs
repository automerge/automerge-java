use jni::{jni_sig, jni_str, objects::JObject, strings::JNIStr, JValue};

use crate::{
    am_value::to_amvalue,
    java_option::{make_empty_option, make_optional},
};

const CONFLICTS_CLASS: &JNIStr = am_classname!("Conflicts");

pub(crate) unsafe fn make_optional_conflicts<'local>(
    env: &mut jni::Env<'local>,
    values: Vec<(automerge::Value<'_>, automerge::ObjId)>,
) -> Result<JObject<'local>, jni::errors::Error> {
    if values.is_empty() {
        make_empty_option(env)
    } else {
        let values_map = env.new_object(jni_str!("java/util/HashMap"), jni_sig!("()V"), &[])?;
        for (value, objid) in values {
            let key = env.new_string(objid.to_string())?;
            let amval = to_amvalue(env, (value, objid))?;
            env.call_method(
                &values_map,
                jni_str!("put"),
                jni_sig!("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"),
                &[(&key).into(), (&amval).into()],
            )?;
        }
        let conflicts = env.alloc_object(CONFLICTS_CLASS)?;
        env.set_field(
            &conflicts,
            jni_str!("values"),
            jni_sig!("Ljava/util/HashMap;"),
            (&values_map).into(),
        )?;
        make_optional(env, JValue::from(&conflicts))
    }
}
