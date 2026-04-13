use automerge_jni_macros::jni_fn;
use jni::sys::jstring;
use jni::Outcome;

// Prefix a JNI type name with the automerge package path
macro_rules! am_classname {
    ($name:literal) => {
        ::jni::jni_str!("org/automerge/", $name)
    };
}

mod conflicts;
mod cursor;
mod document;
mod expand_mark;
mod interop;
mod mark;
mod obj_type;
mod patch_log;
mod patches;
mod path_element;
mod transaction;

mod obj_id;
mod prop;
use jni::strings::JNIStr;
use obj_id::JavaObjId;

mod read_methods;
mod sync;

mod read_ops;

mod am_value;
mod java_option;

const AUTOMERGE_EXCEPTION: &JNIStr = am_classname!("AutomergeException");

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn rustLibVersion(mut env: jni::EnvUnowned) -> jstring {
    let version = match env
        .with_env(|env| env.new_string(env!("CARGO_PKG_VERSION")))
        .into_outcome()
    {
        Outcome::Ok(v) => v,
        _ => todo!(),
    };
    version.into_raw()
}
