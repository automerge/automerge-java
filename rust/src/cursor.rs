use jni::{
    jni_str,
    objects::{JByteArray, JClass, JString},
    strings::JNIString,
    Env, NativeMethod,
};

use crate::interop::{throw_illegal_argument, unwrap_or_throw_amg_exc};

/// Rust-side wrapper for [`automerge::Cursor`].
#[derive(Debug)]
pub struct JavaCursor(automerge::Cursor);

impl AsRef<automerge::Cursor> for JavaCursor {
    fn as_ref(&self) -> &automerge::Cursor {
        &self.0
    }
}

impl From<automerge::Cursor> for JavaCursor {
    fn from(i: automerge::Cursor) -> Self {
        Self(i)
    }
}

impl JavaCursor {
    pub(crate) fn into_cursor<'local>(
        self,
        env: &mut Env<'local>,
    ) -> Result<bindings::Cursor<'local>, jni::errors::Error> {
        let cursor = bindings::Cursor::new(env)?;
        let jbytes = env.byte_array_from_slice(&self.0.to_bytes())?;
        cursor.set_raw(env, &jbytes)?;
        Ok(cursor)
    }

    pub(crate) fn from_cursor<'local>(
        env: &mut Env<'local>,
        obj: bindings::Cursor<'local>,
    ) -> Result<Self, jni::errors::Error> {
        let jbytes = obj.raw(env)?;
        let bytes = env.convert_byte_array(&jbytes)?;
        let cursor: automerge::Cursor =
            unwrap_or_throw_amg_exc::<_, automerge::AutomergeError>(env, bytes.try_into())?;
        Ok(Self(cursor))
    }
}

use crate::bindings;

const _METHODS: &[NativeMethod] = &[
    ams_native! { static extern fn cursor_to_string(obj: bindings::Cursor) -> JString },
    ams_native! { static extern fn cursor_from_string(s: JString) -> bindings::Cursor },
    ams_native! { static extern fn cursor_from_bytes(bytes: jbyte[]) -> bindings::Cursor },
];

fn cursor_to_string<'local>(
    env: &mut Env<'local>,
    _class: JClass<'local>,
    obj: bindings::Cursor<'local>,
) -> jni::errors::Result<JString<'local>> {
    let cursor = JavaCursor::from_cursor(env, obj)?;
    env.new_string(cursor.as_ref().to_string())
}

fn cursor_from_string<'local>(
    env: &mut Env<'local>,
    _class: JClass<'local>,
    s: JString<'local>,
) -> jni::errors::Result<bindings::Cursor<'local>> {
    let s = s.to_string();
    let cursor = match automerge::Cursor::try_from(s) {
        Ok(c) => c,
        Err(e) => {
            throw_illegal_argument(
                env,
                &JNIString::from(format!("invalid cursor string: {}", e)),
            )?;

            return Err(jni::errors::Error::JavaException);
        }
    };
    JavaCursor::from(cursor).into_cursor(env)
}

fn cursor_from_bytes<'local>(
    env: &mut Env<'local>,
    _class: JClass<'local>,
    bytes: JByteArray<'local>,
) -> jni::errors::Result<bindings::Cursor<'local>> {
    let bytes = env.convert_byte_array(&bytes)?;
    match automerge::Cursor::try_from(bytes) {
        Ok(c) => JavaCursor::from(c).into_cursor(env),
        Err(_e) => {
            throw_illegal_argument(env, jni_str!("invalid cursor bytes"))?;

            Err(jni::errors::Error::JavaException)
        }
    }
}
