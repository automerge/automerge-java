use jni::{objects::JObject, sys::jobject};

use automerge as am;

pub(crate) enum JavaObjType {
    Map,
    List,
    Text,
}

pub(crate) const CLASSNAME: &str = am_classname!("ObjectType");

// The ordinal of the various types in the `org.automerge.jni.ObjectType` enum
const MAP_ORDINAL: i32 = 0;
const LIST_ORDINAL: i32 = 1;
const TEXT_ORDINAL: i32 = 2;

const MAP_FIELD_NAME: &str = "MAP";
const LIST_FIELD_NAME: &str = "LIST";
const TEXT_FIELD_NAME: &str = "TEXT";

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

    /// Convert a `JavaObjType` to a `JOBject`
    pub(crate) unsafe fn to_java_enum<'a>(
        &'_ self,
        env: jni::JNIEnv<'a>,
    ) -> Result<JObject<'a>, jni::errors::Error> {
        let field_name = match self {
            Self::Map => MAP_FIELD_NAME,
            Self::List => LIST_FIELD_NAME,
            Self::Text => TEXT_FIELD_NAME,
        };
        let field = env.get_static_field(CLASSNAME, field_name, format!("L{};", CLASSNAME))?;
        field.l()
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
