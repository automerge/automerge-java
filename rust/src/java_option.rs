use jni::{
    objects::{JObject, JValue},
    JNIEnv,
};

pub(crate) fn make_optional<'a>(
    env: &mut jni::JNIEnv<'a>,
    val: JValue,
) -> Result<JObject<'a>, jni::errors::Error> {
    env.call_static_method(
        "java/util/Optional",
        "of",
        "(Ljava/lang/Object;)Ljava/util/Optional;",
        &[val],
    )?
    .l()
}

pub(crate) fn make_empty_option<'a>(
    env: &mut jni::JNIEnv<'a>,
) -> Result<JObject<'a>, jni::errors::Error> {
    env.call_static_method("java/util/Optional", "empty", "()Ljava/util/Optional;", &[])?
        .l()
}

pub(crate) fn make_optional_of<'local, T, F>(
    env: &mut JNIEnv<'local>,
    opt: &Option<T>,
    func: F,
) -> Result<JObject<'local>, jni::errors::Error>
where
    F: for<'a> FnOnce(&mut JNIEnv<'a>, &T) -> Result<JObject<'a>, jni::errors::Error>,
{
    if let Some(val) = opt {
        let val_obj = func(env, val)?;
        make_optional(env, (&val_obj).into())
    } else {
        make_empty_option(env)
    }
}
