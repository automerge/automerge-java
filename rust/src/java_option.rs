use jni::objects::JObject;

use crate::bindings::Optional;

pub(crate) fn make_optional<'local, 'otherlocal, 'a>(
    env: &'a mut jni::Env<'local>,
    val: Option<JObject<'otherlocal>>,
) -> Result<Optional<'local>, jni::errors::Error> {
    match val {
        Some(val) => Optional::of(env, &val),
        None => Optional::empty(env),
    }
}
