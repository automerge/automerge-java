use automerge_jni_macros::jni_fn;
use jni::{
    objects::{JByteArray, JClass, JObject},
    sys::{jbyteArray, jobject, jstring},
    JNIEnv,
};
use samod_core::actors::DocToHubMsg;

use crate::repo::type_mappings::doc_to_hub_msg::java_object_to_doc_to_hub_msg;

/// Create a DocToHubMsg from serialized bytes
/// This throws IllegalArgumentException if the bytes are invalid
#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn createDocToHubMsgFromBytes<'local>(
    mut env: JNIEnv<'local>,
    _class: JClass,
    bytes: jbyteArray,
) -> jobject {
    let bytes = JByteArray::from_raw(bytes);
    let byte_vec = match env.convert_byte_array(bytes) {
        Ok(bytes) => bytes,
        Err(e) => {
            env.throw_new("java/lang/IllegalArgumentException", e.to_string())
                .unwrap();
            return JObject::null().into_raw();
        }
    };

    // Try to deserialize the bytes into a DocToHubMsg
    let _msg = match DocToHubMsg::try_from(byte_vec.as_slice()) {
        Ok(msg) => msg,
        Err(e) => {
            env.throw_new(
                "java/lang/IllegalArgumentException",
                format!("Invalid DocToHubMsg bytes: {:?}", e),
            )
            .unwrap();
            return JObject::null().into_raw();
        }
    };

    // Create the Java object directly using the package-private constructor
    let doc_to_hub_msg_class = match env.find_class(am_classname!("DocToHubMsg")) {
        Ok(class) => class,
        Err(e) => {
            env.throw_new("java/lang/IllegalArgumentException", e.to_string())
                .unwrap();
            return JObject::null().into_raw();
        }
    };

    let byte_array = match env.byte_array_from_slice(&byte_vec) {
        Ok(array) => array,
        Err(e) => {
            env.throw_new("java/lang/IllegalArgumentException", e.to_string())
                .unwrap();
            return JObject::null().into_raw();
        }
    };

    let java_obj = match env.new_object(doc_to_hub_msg_class, "([B)V", &[(&byte_array).into()]) {
        Ok(obj) => obj,
        Err(e) => {
            env.throw_new("java/lang/IllegalArgumentException", e.to_string())
                .unwrap();
            return JObject::null().into_raw();
        }
    };

    java_obj.into_raw()
}

/// Display a DocToHubMsg using the Rust Display implementation
#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn displayDocToHubMsg(
    mut env: JNIEnv,
    _class: JClass,
    msg_obj: JObject,
) -> jstring {
    // Convert Java object to Rust DocToHubMsg
    let msg = match java_object_to_doc_to_hub_msg(&mut env, msg_obj) {
        Ok(msg) => msg,
        Err(e) => {
            env.throw_new("java/lang/IllegalArgumentException", e.to_string())
                .unwrap();
            return JObject::null().into_raw();
        }
    };

    // Use the Display implementation
    let display_string = format!("{:?}", msg);

    // Convert to Java string
    match env.new_string(display_string) {
        Ok(jstr) => jstr.into_raw(),
        Err(e) => {
            env.throw_new("java/lang/IllegalArgumentException", e.to_string())
                .unwrap();
            JObject::null().into_raw()
        }
    }
}
