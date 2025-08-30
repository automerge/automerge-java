use jni::{
    objects::{JObject, JValue},
    JNIEnv,
};
use samod_core::UnixTimestamp;

pub(crate) fn timestamp_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    timestamp: UnixTimestamp,
) -> Result<JObject<'local>, jni::errors::Error> {
    let millis = timestamp.as_millis() as i64;
    let instant_class = env.find_class("java/time/Instant")?;
    env.call_static_method(
        instant_class,
        "ofEpochMilli",
        "(J)Ljava/time/Instant;",
        &[JValue::Long(millis)],
    )?
    .l()
}

pub(crate) fn java_object_to_timestamp<'local>(
    env: &mut JNIEnv<'local>,
    instant: JObject<'local>,
) -> Result<UnixTimestamp, jni::errors::Error> {
    let millis = env.call_method(instant, "toEpochMilli", "()J", &[])?.j()?;
    Ok(UnixTimestamp::from_millis(millis as u128))
}
