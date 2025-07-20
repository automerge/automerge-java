use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use automerge_jni_macros::jni_fn;
use jni::{
    objects::{JObject, JPrimitiveArray},
    sys::{jboolean, jint, jobject},
    JNIEnv,
};

#[derive(Debug)]
pub struct JavaObjId(automerge::ObjId);

impl AsRef<automerge::ObjId> for JavaObjId {
    fn as_ref(&self) -> &automerge::ObjId {
        &self.0
    }
}

impl From<automerge::ObjId> for JavaObjId {
    fn from(i: automerge::ObjId) -> Self {
        Self(i)
    }
}

const CLASSNAME: &str = am_classname!("ObjectId");

impl JavaObjId {
    pub(crate) fn into_raw(self, env: &mut JNIEnv) -> Result<jobject, jni::errors::Error> {
        Ok(self.into_jobject(env)?.into_raw())
    }

    pub(crate) fn into_jobject<'a>(
        self,
        env: &mut JNIEnv<'a>,
    ) -> Result<JObject<'a>, jni::errors::Error> {
        let raw_obj = env.alloc_object(CLASSNAME)?;
        let bytes = self.0.to_bytes();
        let jbytes = env.byte_array_from_slice(&bytes)?;
        env.set_field(&raw_obj, "raw", "[B", (&jbytes).into())?;
        Ok(raw_obj)
    }

    pub(crate) unsafe fn from_raw(
        env: &mut JNIEnv<'_>,
        raw: jobject,
    ) -> Result<Option<Self>, errors::FromRaw> {
        let obj = JObject::from_raw(raw);
        let id_is_null = env
            .is_same_object(&obj, JObject::null())
            .map_err(errors::FromRaw::GetRaw)?;
        if id_is_null {
            return Ok(None);
        }
        let bytes_jobject = env
            .get_field(&obj, "raw", "[B")
            .map_err(errors::FromRaw::GetRaw)?
            .l()
            .map_err(errors::FromRaw::RawPointerNotObject)?;
        let bytes = env
            .convert_byte_array(JPrimitiveArray::from(bytes_jobject))
            .map_err(errors::FromRaw::GetByteArray)?;
        let obj = automerge::ObjId::try_from(bytes.as_slice()).map_err(errors::FromRaw::Invalid)?;
        //Ok(Some(Self(obj)))
        Ok(Some(Self(obj)))
    }
}

/// Get the `ObjId` from a `jobject` or throw an exception and return the given value.
///
/// This macro performs an early return if the `jobject` is null, which means the macro has two
/// forms. The first form, which looks like this:
///
/// ```rust,ignore
/// let obj = obj_id_or_throw!(env, some_obj_id);
/// ```
///
/// Takes a [`&mut jni::JNIEnv`] and a `jobject` and returns a [`JavaObjId`] or throws an exception and
/// early returns a `jobject` from the surrounding function.
///
/// The second form, which looks like this:
///
/// ```rust,ignore
/// let obj = obj_id_or_throw!(env, some_obj_id, false); // the `false` here can be anything
/// ```
///
/// Takes a [`jni::JNIEnv`], a `jobject`, and a value to return from the surrounding function if
/// the `jobject` is null.
macro_rules! obj_id_or_throw {
    ($env:expr, $obj_id:expr) => {
        obj_id_or_throw!($env, $obj_id, JObject::null().into_raw())
    };
    ($env:expr, $obj_id:expr,$returning:expr) => {
        match JavaObjId::from_raw($env, $obj_id) {
            Ok(Some(id)) => id,
            Ok(None) => {
                $env.throw_new(
                    "java/lang/IllegalArgumentException",
                    "Object ID cannot be null",
                )
                .unwrap();
                #[allow(clippy::unused_unit)]
                return $returning;
            }
            Err(e) => {
                use crate::AUTOMERGE_EXCEPTION;
                $env.throw_new(AUTOMERGE_EXCEPTION, format!("{}", e))
                    .unwrap();
                #[allow(clippy::unused_unit)]
                return $returning;
            }
        }
    };
}
pub(crate) use obj_id_or_throw;

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn rootObjectId(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
) -> jni::sys::jobject {
    JavaObjId(automerge::ObjId::Root)
        .into_raw(&mut env)
        .unwrap()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn isRootObjectId(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    obj: jni::sys::jobject,
) -> bool {
    let obj = obj_id_or_throw!(&mut env, obj, false);
    obj.as_ref() == &automerge::ROOT
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn objectIdToString(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    obj: jni::sys::jobject,
) -> jobject {
    let obj = obj_id_or_throw!(&mut env, obj);
    let s = obj.as_ref().to_string();
    let jstr = env.new_string(s).unwrap();
    jstr.into_raw()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn objectIdHash(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    obj: jni::sys::jobject,
) -> jint {
    let obj = obj_id_or_throw!(&mut env, obj, 0);
    let mut hasher = DefaultHasher::new();
    obj.as_ref().hash(&mut hasher);
    hasher.finish() as i32
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn objectIdsEqual(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    left: jni::sys::jobject,
    right: jni::sys::jobject,
) -> jboolean {
    let left = JavaObjId::from_raw(&mut env, left).unwrap();
    let right = JavaObjId::from_raw(&mut env, right).unwrap();
    match (left, right) {
        (None, _) | (_, None) => {
            env.throw_new(
                "java/lang/IllegalArgumentException",
                "Object ID cannot be null",
            )
            .unwrap();
            false.into()
        }
        (Some(left), Some(right)) => (left.as_ref() == right.as_ref()).into(),
    }
}

pub mod errors {
    use super::CLASSNAME;

    #[derive(Debug, thiserror::Error)]
    pub(crate) enum FromRaw {
        #[error("unable to get the 'raw' field: {0} for class {}", CLASSNAME)]
        GetRaw(jni::errors::Error),
        #[error("could not convert the 'raw' pointer to an object: {0}")]
        RawPointerNotObject(jni::errors::Error),
        #[error("error getting byte array from object: {0}")]
        GetByteArray(jni::errors::Error),
        #[error("invalid ID")]
        Invalid(#[from] automerge::ObjIdFromBytesError),
    }
}
