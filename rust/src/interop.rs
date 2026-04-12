use std::sync::MutexGuard;

use am::PatchLog;
use automerge::{self as am, ChangeHash};
use jni::{
    objects::{JObject, JObjectArray, JPrimitiveArray, JValue},
    sys::{jbyteArray, jobject},
    JNIEnv,
};

use crate::AUTOMERGE_EXCEPTION;

pub(crate) const CHANGEHASH_CLASS: &str = am_classname!("ChangeHash");

/// A trait for objects which are represented in Java land as a pointer wrapper
pub(crate) trait JavaPointer: Sized + Send {
    /// Fully qualified Java class name for the pointer wrapper object.
    const POINTER_CLASS: &'static str;

    /// Name of the `long` field in the Java object that holds the raw pointer.
    const POINTER_FIELD: &'static str = "pointer";

    /// Store this Rust value inside a new Java object.
    ///
    /// # Safety
    ///
    /// The caller must ensure the value is valid to store across the JNI boundary.
    unsafe fn store_as_pointer<'local>(
        self,
        env: &mut JNIEnv<'local>,
    ) -> Result<JObject<'local>, jni::errors::Error>
    where
        Self: 'static,
    {
        let obj = env
            .alloc_object(Self::POINTER_CLASS)
            .map_err(|e| jni::errors::Error::from(e))?;
        env.set_rust_field(&obj, Self::POINTER_FIELD, self)
            .map_err(|e| jni::errors::Error::from(e))?;
        Ok(obj)
    }

    /// Borrow the Rust value behind a Java pointer object.
    ///
    /// The pointer remains valid after this call — no ownership is transferred.
    ///
    /// # Safety
    ///
    /// - `ptr` must be a valid jobject with a `POINTER_FIELD` containing a non-null pointer
    ///   previously set by [`store_as_pointer`](Self::store_as_pointer)
    unsafe fn borrow_from_pointer<'a>(
        env: &'a mut JNIEnv<'a>,
        ptr: jobject,
    ) -> Result<MutexGuard<'a, Self>, jni::errors::Error>
    where
        Self: Send + 'static,
    {
        let obj = unsafe { JObject::from_raw(ptr) };
        env.get_rust_field(&obj, Self::POINTER_FIELD)
    }

    /// Take ownership of the Rust value behind a Java pointer object.
    ///
    /// After this call, the Java object must not be used to access the Rust value.
    ///
    /// # Safety
    ///
    /// - `ptr` must be a valid jobject with a `POINTER_FIELD` containing a non-null pointer
    ///   previously set by [`store_as_pointer`](Self::store_as_pointer)
    /// - After this call, the Java object must not be used to access the Rust value
    unsafe fn take_from_pointer<'local>(
        env: &mut JNIEnv<'local>,
        ptr: jobject,
    ) -> Result<Self, jni::errors::Error>
    where
        Self: 'static,
    {
        let obj = unsafe { JObject::from_raw(ptr) };
        env.take_rust_field(&obj, Self::POINTER_FIELD)
    }

    unsafe fn return_to_pointer<'local>(
        self,
        env: &mut JNIEnv<'local>,
        ptr: jobject,
    ) -> Result<(), jni::errors::Error>
    where
        Self: 'static,
    {
        let obj = unsafe { JObject::from_raw(ptr) };
        env.set_rust_field(&obj, Self::POINTER_FIELD, self)
    }
}

impl JavaPointer for automerge::Automerge {
    const POINTER_CLASS: &'static str = am_classname!("AutomergeSys$DocPointer");
}

impl<'a> JavaPointer for automerge::transaction::OwnedTransaction {
    const POINTER_CLASS: &'static str = am_classname!("AutomergeSys$TransactionPointer");
}

impl JavaPointer for automerge::sync::State {
    const POINTER_CLASS: &'static str = am_classname!("AutomergeSys$SyncStatePointer");
}

impl JavaPointer for PatchLog {
    const POINTER_CLASS: &'static str = am_classname!("AutomergeSys$PatchLogPointer");
}

/// Given a pointer to an array of java ChangeHash objects, return a vector of ChangeHashes.
pub(crate) unsafe fn heads_from_jobject(
    env: &mut jni::JNIEnv,
    heads_pointer: jbyteArray,
) -> Result<Vec<ChangeHash>, jni::errors::Error> {
    let heads_pointer = JObjectArray::from_raw(heads_pointer);
    let head_len = env.get_array_length(&heads_pointer)?;
    let mut heads = Vec::with_capacity(head_len as usize);
    for i in 0..head_len {
        let head_obj = env.get_object_array_element(&heads_pointer, i).unwrap();
        let head_bytes_ref =
            JPrimitiveArray::from(env.get_field(head_obj, "hash", "[B").unwrap().l().unwrap());
        let head_bytes = env.convert_byte_array(&head_bytes_ref).unwrap();
        heads.push(automerge::ChangeHash::try_from(head_bytes.as_slice()).unwrap());
    }
    Ok(heads)
}

pub(crate) fn changehash_to_jobject<'local>(
    env: &mut jni::JNIEnv<'local>,
    hash: &ChangeHash,
) -> Result<JObject<'local>, jni::errors::Error> {
    let jhash = env.alloc_object(CHANGEHASH_CLASS)?;
    let byte_array = env.byte_array_from_slice(hash.as_ref())?;
    env.set_field(&jhash, "hash", "[B", (&byte_array).into())
        .unwrap();
    Ok(jhash)
}

pub(crate) fn throw_amg_exc_or_fatal<S: AsRef<str>>(env: &mut jni::JNIEnv, msg: S) {
    throw_or_fatal(env, AUTOMERGE_EXCEPTION, msg);
}

pub(crate) fn throw_or_fatal<S: AsRef<str>>(
    env: &mut jni::JNIEnv,
    exc_class: &'static str,
    msg: S,
) {
    if env.throw_new(exc_class, msg.as_ref()).is_err() {
        eprintln!("Failed to throw exception: {}", msg.as_ref());
        env.fatal_error(format!("Failed to throw exception: {}", msg.as_ref()));
    }
}
