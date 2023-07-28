use automerge_jni_macros::jni_fn;

// Prefix a JNI type name with the automerge package path
macro_rules! am_classname {
    ($name:literal) => {
        concat!("org/automerge/", $name)
    };
}

mod conflicts;
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
use jni::sys::jstring;
use obj_id::JavaObjId;

mod read_methods;
mod sync;

mod read_ops;

mod am_value;
mod java_option;

const AUTOMERGE_EXCEPTION: &str = am_classname!("AutomergeException");

#[allow(clippy::missing_safety_doc)]
#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn rustLibVersion(env: jni::JNIEnv) -> jstring {
    let version = env.new_string(env!("CARGO_PKG_VERSION")).unwrap();
    version.into_raw()
}
