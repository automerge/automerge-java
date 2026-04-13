use jni::{jni_str, objects::JString, sys::jlong};

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
    // Throws an IllegalArgumentException if the index is negative
    pub(crate) fn try_into_prop<'b>(
        self,
        env: &jni::Env<'b>,
    ) -> Result<automerge::Prop, jni::errors::Error> {
        match self {
            Self::String(s) => Ok(automerge::Prop::Map(s.to_string())),
            Self::Idx(i) => {
                let idx = usize::try_from(i).or_else(|_err| {
                    env.with_local_frame(1, |env| {
                        env.throw_new(
                            jni_str!("java/lang/IllegalArgumentException"),
                            jni_str!("index cannot be negative"),
                        )?;
                        Err::<usize, _>(jni::errors::Error::JavaException)
                    })?;
                    Err(jni::errors::Error::JavaException)
                })?;
                Ok(idx.into())
            }
        }
    }
}
