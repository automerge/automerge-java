use jni::objects::{JObject, JValue};

pub(crate) unsafe fn make_optional<'a>(
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

pub(crate) unsafe fn make_empty_option<'a>(
    env: &mut jni::JNIEnv<'a>,
) -> Result<JObject<'a>, jni::errors::Error> {
    env.call_static_method("java/util/Optional", "empty", "()Ljava/util/Optional;", &[])?
        .l()
}
