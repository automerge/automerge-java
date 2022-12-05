use jni::objects::JObject;

use crate::am_value::to_amvalue;

const CONFLICTS_CLASS: &str = am_classname!("Conflicts");

pub(crate) unsafe fn make_optional_conflicts<'a>(
    env: jni::JNIEnv<'a>,
    values: Vec<(automerge::Value<'_>, automerge::ObjId)>,
) -> Option<JObject<'a>> {
    if values.is_empty() {
        None
    } else {
        let values_map = env.new_object("java/util/HashMap", "()V", &[]).unwrap();
        for (value, objid) in values {
            let key = env.new_string(objid.to_string()).unwrap();
            let amval = to_amvalue(&env, (value, objid)).unwrap();
            env.call_method(
                values_map,
                "put",
                "(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;",
                &[key.into(), amval.into()],
            )
            .unwrap();
        }
        let conflicts = env.alloc_object(CONFLICTS_CLASS).unwrap();
        env.set_field(
            conflicts,
            "values",
            "Ljava/util/HashMap;",
            values_map.into(),
        )
        .unwrap();
        Some(conflicts)
    }
}
