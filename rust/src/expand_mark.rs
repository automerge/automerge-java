use am::marks::ExpandMark;
use automerge as am;
use jni::objects::JObject;

// The ordinal of the various types in the `org.automerge.jni.ExpandMark` enum
const BEFORE_ORDINAL: i32 = 0;
const AFTER_ORDINAL: i32 = 1;
const BOTH_ORDINAL: i32 = 2;
const NONE_ORDINAL: i32 = 3;

pub(crate) fn from_java<'a>(
    env: &mut jni::JNIEnv<'a>,
    obj: JObject<'a>,
) -> Result<ExpandMark, FromJavaError> {
    let val = env.call_method(obj, "ordinal", "()I", &[])?.i()?;
    match val {
        BEFORE_ORDINAL => Ok(ExpandMark::Before),
        AFTER_ORDINAL => Ok(ExpandMark::After),
        BOTH_ORDINAL => Ok(ExpandMark::Both),
        NONE_ORDINAL => Ok(ExpandMark::None),
        _ => Err(FromJavaError::InvalidOrdinal(val)),
    }
}

#[derive(thiserror::Error, Debug)]
pub(crate) enum FromJavaError {
    #[error("error loading ExpandMark: {0}")]
    Jni(#[from] jni::errors::Error),
    #[error("the ordinal of the java enum ({0}) was not valid")]
    InvalidOrdinal(i32),
}
