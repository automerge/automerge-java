use am::marks::ExpandMark;
use automerge as am;
use jni::{jni_sig, jni_str, objects::JObject};

// The ordinal of the various types in the `org.automerge.jni.ExpandMark` enum
const BEFORE_ORDINAL: i32 = 0;
const AFTER_ORDINAL: i32 = 1;
const BOTH_ORDINAL: i32 = 2;
const NONE_ORDINAL: i32 = 3;

pub(crate) fn from_java<'a>(
    env: &mut jni::Env<'a>,
    obj: JObject<'a>,
) -> Result<ExpandMark, jni::errors::Error> {
    let val = env
        .call_method(obj, jni_str!("ordinal"), jni_sig!("()I"), &[])?
        .i()?;
    match val {
        BEFORE_ORDINAL => Ok(ExpandMark::Before),
        AFTER_ORDINAL => Ok(ExpandMark::After),
        BOTH_ORDINAL => Ok(ExpandMark::Both),
        NONE_ORDINAL => Ok(ExpandMark::None),
        _ => env.with_local_frame(1, |env| {
            env.throw_new(
                jni_str!("java/lang/IllegalArgumentException"),
                jni_str!("invalid ordinal"),
            )?;
            Err(jni::errors::Error::JavaException)
        }),
    }
}
