use std::sync::MutexGuard;

use am::PatchLog;
use automerge::{self as am, ChangeHash};
use jni::{
    jni_str,
    objects::{JObject, JObjectArray},
    refs::Reference,
    strings::{JNIStr, JNIString},
    sys::jlong,
};

use crate::{bindings, AUTOMERGE_EXCEPTION};

/// A Rust value whose ownership is handed to Java as a `long`-holding wrapper
/// object. The generated [`Wrapper`](Self::Wrapper) type is the `bind_java_type!`
/// binding for that wrapper class (e.g. `bindings::DocPointer`).
///
/// [`Wrapper`]: Self::Wrapper
pub(crate) trait JavaPointer: Sized + Send + 'static {
    /// The `bind_java_type!` wrapper class (e.g. `bindings::DocPointer`) for
    /// this pointer type. Returned by [`store_as_pointer`](Self::store_as_pointer).
    type Wrapper<'local>: Reference<Kind<'local> = Self::Wrapper<'local>> + 'local;

    /// Fully qualified Java class name for the pointer wrapper object.
    const POINTER_CLASS: &'static JNIStr;

    /// Name of the `long` field in the Java object that holds the raw pointer.
    const POINTER_FIELD: &'static JNIStr = jni_str!("pointer");

    /// Store this Rust value inside a new Java wrapper object.
    ///
    /// # Safety
    ///
    /// The caller must ensure the value is valid to store across the JNI boundary.
    unsafe fn store_as_pointer<'local>(
        self,
        env: &mut jni::Env<'local>,
    ) -> Result<Self::Wrapper<'local>, jni::errors::Error> {
        let obj = env.alloc_object(Self::POINTER_CLASS)?;
        env.set_rust_field(&obj, Self::POINTER_FIELD, self)?;
        // SAFETY: we just allocated `obj` as an instance of `POINTER_CLASS`,
        // which the impl guarantees is the Java class bound by `Wrapper`.
        Ok(unsafe { <Self::Wrapper<'local> as Reference>::kind_from_raw(obj.into_raw()) })
    }

    /// Borrow the Rust value behind a Java pointer object.
    ///
    /// # Safety
    ///
    /// - `obj` must be a valid wrapper object whose `POINTER_FIELD` was
    ///   previously set by [`store_as_pointer`](Self::store_as_pointer).
    unsafe fn borrow_from_pointer<'a, 'otherlocal, 'local>(
        env: &'a jni::Env<'local>,
        obj: impl AsRef<JObject<'otherlocal>>,
    ) -> Result<MutexGuard<'local, Self>, jni::errors::Error> {
        env.get_rust_field(obj.as_ref(), Self::POINTER_FIELD)
    }

    /// Take ownership of the Rust value behind a Java pointer object.
    ///
    /// # Safety
    ///
    /// - `obj` must be a valid wrapper object previously populated by
    ///   [`store_as_pointer`](Self::store_as_pointer).
    /// - After this call, the Java object must not be used to access the Rust value.
    unsafe fn take_from_pointer<'local>(
        env: &jni::Env<'local>,
        obj: impl AsRef<JObject<'local>>,
    ) -> Result<Self, jni::errors::Error> {
        env.take_rust_field(obj.as_ref(), Self::POINTER_FIELD)
    }

    /// Re-attach this Rust value to an existing Java pointer wrapper.
    ///
    /// # Safety
    ///
    /// - `obj` must be a valid wrapper object previously emptied by
    ///   [`take_from_pointer`](Self::take_from_pointer).
    unsafe fn return_to_pointer<'local>(
        self,
        env: &jni::Env<'local>,
        obj: impl AsRef<JObject<'local>>,
    ) -> Result<(), jni::errors::Error> {
        env.set_rust_field(obj.as_ref(), Self::POINTER_FIELD, self)
    }
}

impl JavaPointer for automerge::Automerge {
    type Wrapper<'local> = bindings::DocPointer<'local>;
    const POINTER_CLASS: &'static JNIStr = am_classname!("AutomergeSys$DocPointer");
}

impl JavaPointer for automerge::transaction::OwnedTransaction {
    type Wrapper<'local> = bindings::TransactionPointer<'local>;
    const POINTER_CLASS: &'static JNIStr = am_classname!("AutomergeSys$TransactionPointer");
}

impl JavaPointer for automerge::sync::State {
    type Wrapper<'local> = bindings::SyncStatePointer<'local>;
    const POINTER_CLASS: &'static JNIStr = am_classname!("AutomergeSys$SyncStatePointer");
}

impl JavaPointer for PatchLog {
    type Wrapper<'local> = bindings::PatchLogPointer<'local>;
    const POINTER_CLASS: &'static JNIStr = am_classname!("AutomergeSys$PatchLogPointer");
}

/// Given a pointer to an array of java ChangeHash objects, return a vector of ChangeHashes.
pub(crate) fn heads_from_jobject<'local>(
    env: &mut jni::Env<'local>,
    heads_pointer: JObjectArray<'local, bindings::ChangeHash<'local>>,
) -> Result<Vec<ChangeHash>, jni::errors::Error> {
    let head_len = heads_pointer.len(env)?;
    let mut heads = Vec::with_capacity(head_len);
    for i in 0..head_len {
        let jhash = heads_pointer.get_element(env, i)?;
        let head_bytes_ref = jhash.hash(env)?;
        let head_bytes = env.convert_byte_array(&head_bytes_ref)?;
        let hash = match automerge::ChangeHash::try_from(head_bytes.as_slice()) {
            Ok(h) => h,
            Err(e) => {
                let message = JNIString::new(format!("invalid hash at index{}: {}", i, e));
                env.throw_new(jni_str!("java/lang/IllegalArgumentException"), message)?;
                return Err(jni::errors::Error::JavaException);
            }
        };
        heads.push(hash);
    }
    Ok(heads)
}

pub(crate) fn changehash_to_jobject<'local>(
    env: &mut jni::Env<'local>,
    hash: &ChangeHash,
) -> Result<bindings::ChangeHash<'local>, jni::errors::Error> {
    let byte_array = env.byte_array_from_slice(hash.as_ref())?;
    bindings::ChangeHash::new(env, &byte_array)
}

pub(crate) fn heads_to_jobject_array<'local>(
    env: &mut jni::Env<'local>,
    heads: &[automerge::ChangeHash],
) -> Result<JObjectArray<'local, bindings::ChangeHash<'local>>, jni::errors::Error> {
    let heads_arr = env
        .new_object_type_array::<bindings::ChangeHash>(heads.len(), bindings::ChangeHash::null())?;
    for (i, head) in heads.iter().enumerate() {
        let hash = changehash_to_jobject(env, head)?;
        heads_arr.set_element(env, i, hash)?;
    }
    Ok(heads_arr)
}

pub(crate) fn unwrap_or_throw_amg_exc<T, E: std::fmt::Display>(
    env: &jni::Env<'_>,
    val: Result<T, E>,
) -> Result<T, jni::errors::Error> {
    val.or_else(|e| {
        throw_amg_exc(env, e)?;
        Err(jni::errors::Error::JavaException)
    })
}

pub(crate) fn throw_amg_exc<'local, E: std::fmt::Display>(
    env: &jni::Env<'local>,
    e: E,
) -> Result<(), jni::errors::Error> {
    env.with_local_frame(1, |env| {
        let msg_jstr = JNIString::from(e.to_string());
        env.throw_new(AUTOMERGE_EXCEPTION, &msg_jstr)
    })
}

pub(crate) fn read_usize(env: &jni::Env<'_>, val: jlong) -> Result<usize, jni::errors::Error> {
    usize::try_from(val).or_else(|_| {
        env.with_local_frame(1, |env| {
            env.throw_new(
                jni_str!("java/lang/IndexOutOfBoundsException"),
                jni_str!("index cannot be negative"),
            )?;
            Ok::<_, jni::errors::Error>(())
        })?;
        Err(jni::errors::Error::JavaException)
    })
}

pub(crate) fn read_u64(env: &jni::Env<'_>, val: jlong) -> Result<u64, jni::errors::Error> {
    u64::try_from(val).or_else(|_| {
        env.with_local_frame(1, |env| {
            env.throw_new(
                jni_str!("java/lang/IllegalArgumentException"),
                jni_str!("invalid uint value"),
            )?;
            Ok::<_, jni::errors::Error>(())
        })?;
        Err(jni::errors::Error::JavaException)
    })
}
