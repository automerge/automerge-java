use jni::objects::JObject;

use crate::{
    am_value::to_amvalue,
    bindings::{Conflicts, HashMap as JHashMap},
};

pub(crate) unsafe fn make_java_conflicts<'local>(
    env: &mut jni::Env<'local>,
    values: Vec<(automerge::Value<'_>, automerge::ObjId)>,
) -> Result<Conflicts<'local>, jni::errors::Error> {
    let values_map = JHashMap::new(env)?;
    for (value, objid) in values {
        let key = env.new_string(objid.to_string())?;
        let amval = to_amvalue(env, (value, objid))?;
        let amval_obj: JObject = amval.into();
        values_map.put(env, &key, &amval_obj)?;
    }
    let conflicts = Conflicts::new(env)?;
    conflicts.set_values(env, &values_map)?;
    Ok(conflicts)
}
