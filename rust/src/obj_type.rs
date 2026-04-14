use automerge as am;
use jni::{jni_str, objects::JObject, strings::JNIString};

use crate::bindings::ObjectType;

pub(crate) enum JavaObjType {
    Map,
    List,
    Text,
}

// Ordinal positions of variants in the `org.automerge.ObjectType` enum.
const MAP_ORDINAL: i32 = 0;
const LIST_ORDINAL: i32 = 1;
const TEXT_ORDINAL: i32 = 2;

impl JavaObjType {
    pub(crate) fn from_java_enum<'local>(
        env: &mut jni::Env<'local>,
        obj: JObject<'local>,
    ) -> Result<Self, jni::errors::Error> {
        let enum_obj = ObjectType::cast_local(env, obj)?;
        match enum_obj.ordinal(env)? {
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

    pub(crate) fn to_java_enum<'a>(
        &'_ self,
        env: &mut jni::Env<'a>,
    ) -> Result<ObjectType<'a>, jni::errors::Error> {
        match self {
            Self::Map => ObjectType::map(env),
            Self::List => ObjectType::list(env),
            Self::Text => ObjectType::text(env),
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
