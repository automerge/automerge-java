use jni::{objects::JObject, sys::jobject};

use automerge as am;

pub(crate) enum JavaObjType {
    Map,
    List,
    Text,
}

// The ordinal of the various types in the `org.automerge.jni.ObjectType` enum
const MAP_ORDINAL: i32 = 0;
const LIST_ORDINAL: i32 = 1;
const TEXT_ORDINAL: i32 = 2;

impl JavaObjType {
    /// Convert a `jobject` referring to an instance of org.automerge.jni.ObjectType to a
    /// `JavaObjType`
    ///
    /// # Safety
    ///
    /// The `obj` argument must be a valid `jobject` pointer or `null`
    ///
    /// # Errors
    ///
    /// If the object does not have an `ordinal` method which returns an integer or if the ordinal
    /// returned is not recognized.
    pub(crate) unsafe fn from_java_enum(
        env: &jni::JNIEnv,
        obj: jobject,
    ) -> Result<Self, FromJavaError> {
        let obj = JObject::from_raw(obj);
        let val = env
            .call_method(obj, "ordinal", "()I", &[])
            .map_err(FromJavaError::Ordinal)?
            .i()
            .map_err(FromJavaError::OrdinalNotInteger)?;
        match val {
            MAP_ORDINAL => Ok(Self::Map),
            LIST_ORDINAL => Ok(Self::List),
            TEXT_ORDINAL => Ok(Self::Text),
            other => Err(FromJavaError::UnknownOrdinal(other)),
        }
    }
}

impl From<JavaObjType> for am::ObjType {
    fn from(s: JavaObjType) -> Self {
        match s {
            JavaObjType::Map => am::ObjType::Map,
            JavaObjType::List => am::ObjType::List,
            JavaObjType::Text => am::ObjType::Text,
        }
    }
}

impl From<am::ObjType> for JavaObjType {
    fn from(o: am::ObjType) -> Self {
        match o {
            am::ObjType::Map | am::ObjType::Table => JavaObjType::Map,
            am::ObjType::List => JavaObjType::List,
            am::ObjType::Text => JavaObjType::Text,
        }
    }
}

#[derive(Debug, thiserror::Error)]
pub(crate) enum FromJavaError {
    #[error("unable to call the 'ordinal' method: {0}")]
    Ordinal(jni::errors::Error),
    #[error("unable to convert the result of the 'ordinal' method to an integer: {0}")]
    OrdinalNotInteger(jni::errors::Error),
    #[error("unknown ordinal {0}")]
    UnknownOrdinal(i32),
}
