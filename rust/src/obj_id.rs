use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use automerge_jni_macros::jni_fn;
use jni::{
    objects::JObject,
    sys::{jboolean, jbyteArray, jint, jobject},
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
    pub(crate) fn into_raw(self, env: &JNIEnv) -> Result<jobject, jni::errors::Error> {
        Ok(self.into_jobject(env)?.into_raw())
    }

    pub(crate) fn into_jobject<'a>(
        self,
        env: &JNIEnv<'a>,
    ) -> Result<JObject<'a>, jni::errors::Error> {
        let raw_obj = env.alloc_object(CLASSNAME)?;
        let bytes = self.0.to_bytes();
        let jbytes = env.byte_array_from_slice(&bytes)?;
        env.set_field(
            raw_obj,
            "raw",
            "[B",
            unsafe { JObject::from_raw(jbytes) }.into(),
        )?;
        Ok(raw_obj)
    }

    pub(crate) unsafe fn from_raw(env: &JNIEnv<'_>, raw: jobject) -> Result<Self, errors::FromRaw> {
        let obj = JObject::from_raw(raw);
        let bytes_jobject = env
            .get_field(obj, "raw", "[B")
            .map_err(errors::FromRaw::GetRaw)?
            .l()
            .map_err(errors::FromRaw::RawPointerNotObject)?
            .into_raw() as jbyteArray;
        let arr = env
            .get_byte_array_elements(bytes_jobject, jni::objects::ReleaseMode::NoCopyBack)
            .map_err(errors::FromRaw::GetByteArray)?;
        let bytes =
            std::slice::from_raw_parts(arr.as_ptr() as *const u8, arr.size().unwrap() as usize);
        let obj: automerge::ObjId = bytes.try_into()?;
        Ok(Self(obj))
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn rootObjectId(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
) -> jni::sys::jobject {
    JavaObjId(automerge::ObjId::Root).into_raw(&env).unwrap()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn isRootObjectId(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    obj: jni::sys::jobject,
) -> bool {
    let obj = JavaObjId::from_raw(&env, obj).unwrap();
    obj.as_ref() == &automerge::ROOT
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn objectIdToString(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    obj: jni::sys::jobject,
) -> jobject {
    let obj = JavaObjId::from_raw(&env, obj).unwrap();
    let s = obj.as_ref().to_string();
    let jstr = env.new_string(s).unwrap();
    jstr.into_raw()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn objectIdHash(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    left: jni::sys::jobject,
) -> jint {
    let obj = JavaObjId::from_raw(&env, left).unwrap();
    let mut hasher = DefaultHasher::new();
    obj.as_ref().hash(&mut hasher);
    hasher.finish() as i32
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn objectIdsEqual(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    left: jni::sys::jobject,
    right: jni::sys::jobject,
) -> jboolean {
    let left = JavaObjId::from_raw(&env, left).unwrap();
    let right = JavaObjId::from_raw(&env, right).unwrap();
    (left.as_ref() == right.as_ref()).into()
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
