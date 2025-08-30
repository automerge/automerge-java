use jni::objects::JObject;
use jni::JNIEnv;
use samod_core::network::ConnDirection;

pub(crate) const CONN_DIRECTION_CLASS: &str = am_classname!("ConnDirection");

pub(crate) fn conn_direction_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    conn_direction: &ConnDirection,
) -> Result<JObject<'local>, jni::errors::Error> {
    let enum_class = env.find_class(CONN_DIRECTION_CLASS)?;
    let field_name = match conn_direction {
        ConnDirection::Outgoing => "OUTGOING",
        ConnDirection::Incoming => "INCOMING",
    };
    let enum_value = env.get_static_field(
        enum_class,
        field_name,
        format!("L{};", CONN_DIRECTION_CLASS),
    )?;
    Ok(enum_value.l()?)
}

pub(crate) fn java_object_to_conn_direction(
    env: &mut JNIEnv,
    obj: JObject,
) -> Result<ConnDirection, jni::errors::Error> {
    let name_result = env.call_method(obj, "name", "()Ljava/lang/String;", &[])?;
    let name_obj = name_result.l()?;
    let name_string = env.get_string((&name_obj).into())?;
    let name_str: String = name_string.into();
    match name_str.as_str() {
        "OUTGOING" => Ok(ConnDirection::Outgoing),
        "INCOMING" => Ok(ConnDirection::Incoming),
        _ => Err(jni::errors::Error::JavaException),
    }
}
