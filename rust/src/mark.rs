use automerge as am;
use jni::objects::JObject;

use crate::am_value;

pub(crate) const MARK_CLASSNAME: &str = am_classname!("Mark");

pub(crate) fn mark_to_java<'a>(
    env: &jni::JNIEnv<'a>,
    mark: &am::marks::Mark,
) -> jni::errors::Result<JObject<'a>> {
    let mark_class = env.find_class(MARK_CLASSNAME)?;
    let value = am_value::scalar_to_amvalue(env, mark.value())?;
    let name = env.new_string(mark.name())?;

    env.new_object(
        mark_class,
        format!("(JJLjava/lang/String;L{};)V", am_classname!("AmValue")),
        &[
            (mark.start as i64).into(),
            (mark.end as i64).into(),
            name.into(),
            value.into(),
        ],
    )
}
