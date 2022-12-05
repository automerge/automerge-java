use jni::{objects::JString, sys::jlong};

pub(crate) enum JProp<'a> {
    String(JString<'a>),
    Idx(jlong),
}

impl<'a> From<JString<'a>> for JProp<'a> {
    fn from(s: JString<'a>) -> Self {
        JProp::String(s)
    }
}

impl<'a> From<jlong> for JProp<'a> {
    fn from(i: jlong) -> Self {
        JProp::Idx(i)
    }
}

impl<'a> JProp<'a> {
    pub(crate) fn try_into_prop(self, env: jni::JNIEnv<'a>) -> Result<automerge::Prop, PropError> {
        match self {
            Self::String(s) => {
                let jstr = env.get_string(s)?;
                let s = jstr.to_str().map_err(|_| PropError::BadKey)?;
                Ok(automerge::Prop::Map(s.to_string()))
            }
            Self::Idx(i) => Ok(usize::try_from(i).map_err(|_| PropError::BadIndex)?.into()),
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum PropError {
    #[error("index must not be negative")]
    BadIndex,
    #[error(transparent)]
    Jni(#[from] jni::errors::Error),
    #[error("not a valid UTF-8 string")]
    BadKey,
}
