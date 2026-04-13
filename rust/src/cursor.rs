use automerge_jni_macros::jni_fn;
use jni::{
    errors::ThrowRuntimeExAndDefault,
    jni_sig, jni_str,
    objects::{JByteArray, JClass, JObject, JString},
    strings::{JNIStr, JNIString},
    Env,
};

use crate::interop::{set_array_field, unwrap_or_throw_amg_exc};

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

const CLASSNAME: &JNIStr = am_classname!("Cursor");

impl Cursor {
    pub(crate) fn into_jobject<'local>(
        self,
        env: &mut Env<'local>,
    ) -> Result<JObject<'local>, jni::errors::Error> {
        let raw_obj = env.alloc_object(CLASSNAME)?;
        let bytes = self.0.to_bytes();
        let jbytes = env.byte_array_from_slice(&bytes)?;
        unsafe {
            set_array_field(
                env,
                &raw_obj,
                jni_str!("raw"),
                jni_sig!("[B"),
                (&jbytes).into(),
            )?
        };
        Ok(raw_obj)
    }

    pub(crate) fn from_jobject<'local>(
        env: &mut Env<'local>,
        obj: JObject<'local>,
    ) -> Result<Self, jni::errors::Error> {
        let bytes_jobject = env.get_field(obj, jni_str!("raw"), jni_sig!("[B"))?.l()?;
        let jbytearray = JByteArray::cast_local(env, bytes_jobject)?;
        let bytes = env.convert_byte_array(&jbytearray)?;
        let cursor: automerge::Cursor =
            unwrap_or_throw_amg_exc::<_, automerge::AutomergeError>(env, bytes.try_into())?;
        Ok(Self(cursor))
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn cursorToString<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    obj: JObject<'local>,
) -> JString<'local> {
    env.with_env(|env| {
        let cursor = Cursor::from_jobject(env, obj)?;
        let s = cursor.as_ref().to_string();
        env.new_string(s)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn cursorFromString<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    jstring: JString<'local>,
) -> JObject<'local> {
    env.with_env(|env| {
        let s = jstring.to_string();
        let cursor = match automerge::Cursor::try_from(s) {
            Ok(c) => c,
            Err(e) => {
                env.throw_new(
                    jni_str!("java/lang/IllegalArgumentException"),
                    JNIString::from(format!("invalid cursor string: {}", e)),
                )?;
                return Err(jni::errors::Error::JavaException);
            }
        };
        Cursor::from(cursor).into_jobject(env)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn cursorFromBytes<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    bytes: JByteArray<'local>,
) -> JObject<'local> {
    env.with_env(|env| {
        let bytes = env.convert_byte_array(&bytes)?;
        match automerge::Cursor::try_from(bytes) {
            Ok(c) => Cursor::from(c).into_jobject(env),
            Err(_e) => {
                env.throw_new(
                    jni_str!("java/lang/IllegalArgumentException"),
                    jni_str!("invalid cursor bytes"),
                )?;
                Err(jni::errors::Error::JavaException)
            }
        }
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}
