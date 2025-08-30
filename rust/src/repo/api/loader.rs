use crate::repo::type_mappings::loader_step_result::loader_state_to_java_object;
use crate::AUTOMERGE_EXCEPTION;
use crate::{
    interop::AsPointerObj, repo::type_mappings::io_result::java_object_to_io_result_storage,
};
use automerge_jni_macros::jni_fn;
use jni::{
    objects::{JObject, JString},
    sys::jobject,
    JNIEnv,
};
use rand::{rngs::StdRng, SeedableRng};
use samod_core::{PeerId, SamodLoader, UnixTimestamp};

/// Create a new SamodLoader instance
#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn createSamodLoader<'local>(
    mut env: JNIEnv<'local>,
    _class: jni::objects::JClass,
    peer_id_str: JString,
) -> jobject {
    // Convert Java string to Rust string
    let peer_id_string: String = match env.get_string(&peer_id_str) {
        Ok(s) => s.into(),
        Err(e) => {
            env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
            return JObject::null().into_raw();
        }
    };

    // Create PeerId from string
    let peer_id = PeerId::from_string(peer_id_string);

    // Create SamodLoader
    let loader = SamodLoader::new(peer_id);

    // Convert to pointer object
    match loader.to_pointer_obj(&mut env) {
        Ok(ptr) => ptr.into_raw(),
        Err(e) => {
            env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
            JObject::null().into_raw()
        }
    }
}

/// Step the SamodLoader and return the result
#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn stepSamodLoader<'local>(
    mut env: JNIEnv<'local>,
    _class: jni::objects::JClass,
    loader_ptr: jobject,
    timestamp: i64,
) -> jobject {
    // Get the loader from pointer
    let loader = match SamodLoader::from_pointer_obj(&mut env, loader_ptr) {
        Ok(loader) => loader,
        Err(e) => {
            env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
            return JObject::null().into_raw();
        }
    };

    // Create RNG for the step operation
    let mut rng = StdRng::from_rng(&mut rand::rng());

    // Convert timestamp to UnixTimestamp
    let unix_timestamp = UnixTimestamp::from_millis(timestamp as u128);

    // Call step method
    let loader_state = loader.step(&mut rng, unix_timestamp);

    // Convert result to Java object
    match loader_state_to_java_object(&mut env, loader_state) {
        Ok(obj) => obj.into_raw(),
        Err(e) => {
            env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
            JObject::null().into_raw()
        }
    }
}

/// Provide an IO result to the SamodLoader
#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn provideSamodLoaderIoResult<'local>(
    mut env: JNIEnv<'local>,
    _class: jni::objects::JClass,
    loader_ptr: jobject,
    io_result_obj: jobject,
) {
    // Get the loader from pointer
    let loader = match SamodLoader::from_pointer_obj(&mut env, loader_ptr) {
        Ok(loader) => loader,
        Err(e) => {
            env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
            return;
        }
    };

    // Convert Java IoResult to Rust IoResult
    let io_result =
        java_object_to_io_result_storage(&mut env, JObject::from_raw(io_result_obj)).unwrap();

    // Provide the IO result to the loader
    loader.provide_io_result(io_result);
}

/// Free a SamodLoader instance
#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn freeSamodLoader<'local>(
    mut env: JNIEnv<'local>,
    _class: jni::objects::JClass,
    loader_ptr: jobject,
) {
    // Take ownership of the loader to free it
    match SamodLoader::owned_from_pointer_obj(&mut env, loader_ptr) {
        Ok(_loader) => {
            // Loader is automatically dropped when it goes out of scope
        }
        Err(e) => {
            env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
        }
    }
}
