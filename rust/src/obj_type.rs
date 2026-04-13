use jni::{
    jni_sig, jni_str,
    objects::JObject,
    signature::RuntimeFieldSignature,
    strings::{JNIStr, JNIString},
};

use automerge as am;

pub(crate) enum JavaObjType {
    Map,
    List,
    Text,
}

pub(crate) const CLASSNAME: &JNIStr = am_classname!("ObjectType");

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
    pub(crate) unsafe fn from_java_enum<'local>(
        env: &mut jni::Env<'local>,
        obj: JObject<'local>,
    ) -> Result<Self, jni::errors::Error> {
        let val = env
            .call_method(obj, jni_str!("ordinal"), jni_sig!("()I"), &[])?
            .i()?;
        match val {
            MAP_ORDINAL => Ok(Self::Map),
            LIST_ORDINAL => Ok(Self::List),
            TEXT_ORDINAL => Ok(Self::Text),
            other => env.with_local_frame(1, |env| {
                let msg = JNIString::from(format!("unknown ordinal: {}", other));
                env.throw_new(jni_str!("java/lang/IllegalArgumentException"), msg)?;
                Err(jni::errors::Error::JavaException)
            }),
        }
    }

    /// Convert a `JavaObjType` to a `JOBject`
    pub(crate) unsafe fn to_java_enum<'a>(
        &'_ self,
        env: &mut jni::Env<'a>,
    ) -> Result<JObject<'a>, jni::errors::Error> {
        let field_name = match self {
            Self::Map => MAP_FIELD_NAME,
            Self::List => LIST_FIELD_NAME,
            Self::Text => TEXT_FIELD_NAME,
        };
        // TODO: replace this with jni_sig!?
        let field_sig = RuntimeFieldSignature::from_str(format!("L{CLASSNAME};")).unwrap();
        let field = env.get_static_field(
            CLASSNAME,
            JNIString::from(field_name),
            field_sig.field_signature(),
        )?;
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
