use jni::{
    jni_sig, jni_str,
    objects::{JObject, JValue},
};

pub(crate) fn make_optional<'local, 'otherlocal, 'a>(
    env: &'a mut jni::Env<'local>,
    val: JValue<'otherlocal>,
) -> Result<JObject<'local>, jni::errors::Error> {
    env.call_static_method(
        jni_str!("java/util/Optional"),
        jni_str!("of"),
        jni_sig!("(Ljava/lang/Object;)Ljava/util/Optional;"),
        &[val],
    )?
    .l()
}

pub(crate) fn make_empty_option<'a>(
    env: &mut jni::Env<'a>,
) -> Result<JObject<'a>, jni::errors::Error> {
    env.call_static_method(
        jni_str!("java/util/Optional"),
        jni_str!("empty"),
        jni_sig!("()Ljava/util/Optional;"),
        &[],
    )?
    .l()
}
