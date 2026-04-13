use automerge as am;
use jni::{objects::JObject, signature::RuntimeMethodSignature, strings::JNIStr};

use crate::am_value;

pub(crate) const MARK_CLASSNAME: &JNIStr = am_classname!("Mark");

pub(crate) fn mark_to_java<'local>(
    env: &mut jni::Env<'local>,
    mark: &am::marks::Mark,
) -> jni::errors::Result<JObject<'local>> {
    let mark_class = env.find_class(MARK_CLASSNAME)?;
    let value = am_value::scalar_to_amvalue(env, mark.value())?;
    let name = env.new_string(mark.name())?;

    // TODO: replace this with jni_sig
    let constructor_sig = RuntimeMethodSignature::from_str(format!(
        "(JJLjava/lang/String;L{};)V",
        am_classname!("AmValue")
    ))?;

    env.new_object(
        mark_class,
        constructor_sig.method_signature(),
        &[
            (mark.start as i64).into(),
            (mark.end as i64).into(),
            (&name).into(),
            (&value).into(),
        ],
    )
}
