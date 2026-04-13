use std::sync::MutexGuard;

use am::PatchLog;
use automerge::{self as am, ChangeHash};
use jni::{
    jni_sig, jni_str,
    objects::{JObject, JObjectArray, JPrimitiveArray, JValue},
    refs::IntoAuto,
    signature::FieldSignature,
    strings::{JNIStr, JNIString},
    sys::jlong,
};

use crate::AUTOMERGE_EXCEPTION;

pub(crate) const CHANGEHASH_CLASS: &JNIStr = am_classname!("ChangeHash");

/// A trait for objects which are represented in Java land as a pointer wrapper
pub(crate) trait JavaPointer: Sized + Send {
    /// Fully qualified Java class name for the pointer wrapper object.
    const POINTER_CLASS: &'static JNIStr;

    /// Name of the `long` field in the Java object that holds the raw pointer.
    const POINTER_FIELD: &'static JNIStr = jni_str!("pointer");

    /// Store this Rust value inside a new Java object.
    ///
    /// # Safety
    ///
    /// The caller must ensure the value is valid to store across the JNI boundary.
    unsafe fn store_as_pointer<'local>(
        self,
        env: &mut jni::Env<'local>,
    ) -> Result<JObject<'local>, jni::errors::Error>
    where
        Self: 'static,
    {
        let obj = env.alloc_object(Self::POINTER_CLASS)?;
        env.set_rust_field(&obj, Self::POINTER_FIELD, self)?;
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
    unsafe fn borrow_from_pointer<'a, 'otherlocal, 'local>(
        env: &'a jni::Env<'local>,
        obj: JObject<'otherlocal>,
    ) -> Result<MutexGuard<'local, Self>, jni::errors::Error>
    where
        Self: Send + 'static,
    {
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
        env: &jni::Env<'local>,
        obj: JObject<'local>,
    ) -> Result<Self, jni::errors::Error>
    where
        Self: 'static,
    {
        env.take_rust_field(&obj, Self::POINTER_FIELD)
    }

    unsafe fn return_to_pointer<'local>(
        self,
        env: &jni::Env<'local>,
        obj: JObject<'local>,
    ) -> Result<(), jni::errors::Error>
    where
        Self: 'static,
    {
        env.set_rust_field(&obj, Self::POINTER_FIELD, self)
    }
}

impl JavaPointer for automerge::Automerge {
    const POINTER_CLASS: &'static JNIStr = am_classname!("AutomergeSys$DocPointer");
}

impl JavaPointer for automerge::transaction::OwnedTransaction {
    const POINTER_CLASS: &'static JNIStr = am_classname!("AutomergeSys$TransactionPointer");
}

impl JavaPointer for automerge::sync::State {
    const POINTER_CLASS: &'static JNIStr = am_classname!("AutomergeSys$SyncStatePointer");
}

impl JavaPointer for PatchLog {
    const POINTER_CLASS: &'static JNIStr = am_classname!("AutomergeSys$PatchLogPointer");
}

/// Given a pointer to an array of java ChangeHash objects, return a vector of ChangeHashes.
pub(crate) unsafe fn heads_from_jobject<'local>(
    env: &mut jni::Env<'local>,
    heads_pointer: JObjectArray<'local>,
) -> Result<Vec<ChangeHash>, jni::errors::Error> {
    let head_len = heads_pointer.len(env)?;
    let mut heads = Vec::with_capacity(head_len);
    for i in 0..head_len {
        let head_obj = heads_pointer.get_element(env, i)?;
        let hash = env
            .get_field(head_obj, jni_str!("hash"), jni_sig!("[B"))?
            .l()?
            .into_raw();
        let head_bytes_ref = JPrimitiveArray::from_raw(env, hash);
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

/// Set a field that holds an array type (e.g. `[B`).
///
/// This is needed because `Env::set_field` has a type check that compares
/// `JValue::java_type()` against the field signature's type, but `JValue::Object`
/// produces `JavaType::Object` while array signatures like `[B` produce
/// `JavaType::Array`. Since Java arrays _are_ objects, this check is overly
/// strict and causes a `WrongJValueType` error. We use `set_field_unchecked`
/// to bypass it.
///
/// This should be fixed if https://github.com/jni-rs/jni-rs/pull/811 is merged
pub(crate) unsafe fn set_array_field<'other_local, 'sig, O, N, S>(
    env: &mut jni::Env<'_>,
    obj: O,
    name: N,
    sig: S,
    value: JValue,
) -> Result<(), jni::errors::Error>
where
    O: AsRef<JObject<'other_local>>,
    N: AsRef<JNIStr>,
    S: AsRef<FieldSignature<'sig>>,
{
    let obj = obj.as_ref();
    let class = env.get_object_class(obj)?.auto();
    env.set_field_unchecked(obj, (&class, name, sig), value)
}

pub(crate) fn changehash_to_jobject<'local>(
    env: &mut jni::Env<'local>,
    hash: &ChangeHash,
) -> Result<JObject<'local>, jni::errors::Error> {
    let jhash = env.alloc_object(CHANGEHASH_CLASS)?;
    let byte_array = env.byte_array_from_slice(hash.as_ref())?;
    unsafe {
        set_array_field(
            env,
            &jhash,
            jni_str!("hash"),
            jni_sig!("[B"),
            (&byte_array).into(),
        )?
    };
    Ok(jhash)
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
