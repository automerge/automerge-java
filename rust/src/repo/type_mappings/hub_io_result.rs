use jni::objects::JObject;
use jni::JNIEnv;
use samod_core::actors::hub::io::HubIoResult;

pub(crate) fn java_object_to_hub_io_result<'local>(
    env: &mut JNIEnv<'local>,
    obj: JObject<'local>,
) -> Result<HubIoResult, jni::errors::Error> {
    let name_result = env.call_method(obj, "name", "()Ljava/lang/String;", &[])?;
    let name_obj = name_result.l()?;
    let name_string = env.get_string((&name_obj).into())?;
    let name_str: String = name_string.into();
    match name_str.as_str() {
        "SEND" => Ok(HubIoResult::Send),
        "DISCONNECT" => Ok(HubIoResult::Disconnect),
        _ => Err(jni::errors::Error::JavaException),
    }
}
