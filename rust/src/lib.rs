use jni::objects::{JClass, JString};
use jni::NativeMethod;
mod macros;

mod bindings;
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

const _METHODS: &[NativeMethod] = &[ams_native! { static extern fn rust_lib_version() -> JString }];

fn rust_lib_version<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
) -> jni::errors::Result<JString<'local>> {
    env.new_string(env!("CARGO_PKG_VERSION"))
}
