use automerge_jni_macros::jni_fn;
use jni::{
    objects::{JObject, JString},
    sys::{jbyteArray, jobject},
    JNIEnv,
};

#[derive(Debug)]
pub struct Cursor(automerge::Cursor);

impl AsRef<automerge::Cursor> for Cursor {
    fn as_ref(&self) -> &automerge::Cursor {
        &self.0
    }
}

impl From<automerge::Cursor> for Cursor {
    fn from(i: automerge::Cursor) -> Self {
        Self(i)
    }
}

const CLASSNAME: &str = am_classname!("Cursor");

impl Cursor {
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
        let cursor: automerge::Cursor = bytes.try_into().map_err(errors::FromRaw::Invalid)?;
        Ok(Self(cursor))
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn cursorToString(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    obj: jni::sys::jobject,
) -> jni::sys::jobject {
    let cursor = Cursor::from_raw(&env, obj).unwrap();
    let s = cursor.as_ref().to_string();
    let jstr = env.new_string(s).unwrap();
    jstr.into_raw()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn cursorFromString(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    s: jni::sys::jstring,
) -> jobject {
    let s = env.get_string(JString::from_raw(s)).unwrap();
    let Ok(s) = s.to_str() else {
        env.throw_new(
            "java/lang/IllegalArgumentException",
            "invalid cursor string",
        )
        .unwrap();
        return JObject::null().into_raw();
    };
    let Ok(cursor) = automerge::Cursor::try_from(s) else {
        env.throw_new(
            "java/lang/IllegalArgumentException",
            "invalid cursor string",
        )
        .unwrap();
        return JObject::null().into_raw();
    };
    Cursor::from(cursor).into_raw(&env).unwrap()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn cursorFromBytes(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    bytes: jni::sys::jbyteArray,
) -> jobject {
    let bytes = env
        .get_byte_array_elements(bytes, jni::objects::ReleaseMode::NoCopyBack)
        .unwrap();
    let bytes =
        std::slice::from_raw_parts(bytes.as_ptr() as *const u8, bytes.size().unwrap() as usize);
    let Ok(cursor) = automerge::Cursor::try_from(bytes) else {
        // throw IllegalArgumentException
        env.throw_new("java/lang/IllegalArgumentException", "invalid cursor bytes")
            .unwrap();
        return JObject::null().into_raw();
    };
    Cursor::from(cursor).into_raw(&env).unwrap()
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
        Invalid(automerge::AutomergeError),
    }
}
