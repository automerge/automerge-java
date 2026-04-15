use am::marks::ExpandMark;
use automerge as am;
use jni::jni_str;

use crate::{bindings, interop::throw_illegal_argument};

// Ordinal positions of variants in the `org.automerge.ExpandMark` enum.
const BEFORE_ORDINAL: i32 = 0;
const AFTER_ORDINAL: i32 = 1;
const BOTH_ORDINAL: i32 = 2;
const NONE_ORDINAL: i32 = 3;

pub(crate) fn from_java<'a>(
    env: &mut jni::Env<'a>,
    enum_obj: bindings::ExpandMark<'a>,
) -> Result<ExpandMark, jni::errors::Error> {
    match enum_obj.ordinal(env)? {
        BEFORE_ORDINAL => Ok(ExpandMark::Before),
        AFTER_ORDINAL => Ok(ExpandMark::After),
        BOTH_ORDINAL => Ok(ExpandMark::Both),
        NONE_ORDINAL => Ok(ExpandMark::None),
        _ => env.with_local_frame(1, |env| {
            throw_illegal_argument(env, jni_str!("invalid ordinal"))?;

            Err(jni::errors::Error::JavaException)
        }),
    }
}
