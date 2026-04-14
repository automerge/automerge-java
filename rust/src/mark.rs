use automerge as am;

use crate::{am_value, bindings::Mark};

pub(crate) fn mark_to_java<'local>(
    env: &mut jni::Env<'local>,
    mark: &am::marks::Mark,
) -> jni::errors::Result<Mark<'local>> {
    let value = am_value::scalar_to_amvalue(env, mark.value())?;
    let name = env.new_string(mark.name())?;
    Mark::new(env, mark.start as i64, mark.end as i64, &name, &value)
}
