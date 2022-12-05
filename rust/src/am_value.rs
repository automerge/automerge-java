use automerge::{self as am, ObjType};
use jni::objects::{JObject, JValue};

use crate::JavaObjId;

const FLOAT_CLASS: &str = am_classname!("AmValue$F64");
const BYTE_CLASS: &str = am_classname!("AmValue$Bytes");
const STR_CLASS: &str = am_classname!("AmValue$Str");
const INT_CLASS: &str = am_classname!("AmValue$Int");
const UINT_CLASS: &str = am_classname!("AmValue$UInt");
const BOOL_CLASS: &str = am_classname!("AmValue$Bool");
const NULL_CLASS: &str = am_classname!("AmValue$Null");
const COUNTER_CLASS: &str = am_classname!("AmValue$Counter");
const TIMESTAMP_CLASS: &str = am_classname!("AmValue$Timestamp");
const UNKNOWN_CLASS: &str = am_classname!("AmValue$Unknown");
const MAP_CLASS: &str = am_classname!("AmValue$Map");
const LIST_CLASS: &str = am_classname!("AmValue$List");
const TEXT_CLASS: &str = am_classname!("AmValue$Text");

pub(crate) unsafe fn to_optional_amvalue<'a>(
    env: &jni::JNIEnv<'a>,
    val: Option<(automerge::Value, automerge::ObjId)>,
) -> Result<JObject<'a>, jni::errors::Error> {
    match val {
        Some(val) => {
            let val = to_amvalue(env, val)?;
            let opt = env
                .call_static_method(
                    "java/util/Optional",
                    "of",
                    "(Ljava/lang/Object;)Ljava/util/Optional;",
                    &[val.into()],
                )?
                .l()?;
            Ok(opt)
        }
        None => {
            let opt = env
                .call_static_method("java/util/Optional", "empty", "()Ljava/util/Optional;", &[])?
                .l()?;
            Ok(opt)
        }
    }
}

pub(crate) unsafe fn to_amvalue<'a>(
    env: &jni::JNIEnv<'a>,
    val: (automerge::Value, automerge::ObjId),
) -> Result<JObject<'a>, jni::errors::Error> {
    match val {
        (automerge::Value::Scalar(s), _) => scalar_to_amvalue(env, s.as_ref()),
        (automerge::Value::Object(objtype), oid) => {
            let oid = JavaObjId::from(oid).into_jobject(env)?;
            match objtype {
                ObjType::Map | ObjType::Table => {
                    let amval = env.alloc_object(MAP_CLASS)?;
                    env.set_field(
                        amval,
                        "id",
                        format!("L{};", am_classname!("ObjectId")),
                        oid.into(),
                    )?;
                    Ok(amval)
                }
                ObjType::List => {
                    let amval = env.alloc_object(LIST_CLASS)?;
                    env.set_field(
                        amval,
                        "id",
                        format!("L{};", am_classname!("ObjectId")),
                        oid.into(),
                    )?;
                    Ok(amval)
                }
                ObjType::Text => {
                    let amval = env.alloc_object(TEXT_CLASS)?;
                    env.set_field(
                        amval,
                        "id",
                        format!("L{};", am_classname!("ObjectId")),
                        oid.into(),
                    )?;
                    Ok(amval)
                }
            }
        }
    }
}

pub(crate) fn scalar_to_amvalue<'a>(
    env: &jni::JNIEnv<'a>,
    val: &am::ScalarValue,
) -> jni::errors::Result<JObject<'a>> {
    match val {
        am::ScalarValue::F64(v) => {
            let amval = env.alloc_object(FLOAT_CLASS)?;
            env.set_field(amval, "value", "D", JValue::Double(*v))?;
            Ok(amval)
        }
        am::ScalarValue::Bytes(v) => {
            let amval = env.alloc_object(BYTE_CLASS)?;
            let arr = env.byte_array_from_slice(v)?;
            env.set_field(
                amval,
                "value",
                "[B",
                unsafe { JObject::from_raw(arr) }.into(),
            )?;
            Ok(amval)
        }
        am::ScalarValue::Str(s) => {
            let s = env.new_string(s.as_str())?;
            let amval = env.alloc_object(STR_CLASS)?;
            env.set_field(amval, "value", "Ljava/lang/String;", s.into())?;
            Ok(amval)
        }
        am::ScalarValue::Int(i) => {
            let amval = env.alloc_object(INT_CLASS)?;
            env.set_field(amval, "value", "J", JValue::Long(*i))?;
            Ok(amval)
        }
        am::ScalarValue::Uint(i) => {
            let amval = env.alloc_object(UINT_CLASS)?;
            env.set_field(amval, "value", "J", JValue::Long(*i as i64))?;
            Ok(amval)
        }
        am::ScalarValue::Boolean(val) => {
            let amval = env.alloc_object(BOOL_CLASS)?;
            env.set_field(amval, "value", "Z", JValue::Bool(*val as u8))?;
            Ok(amval)
        }
        am::ScalarValue::Null => {
            let amval = env.alloc_object(NULL_CLASS)?;
            Ok(amval)
        }
        am::ScalarValue::Counter(c) => {
            let amval = env.alloc_object(COUNTER_CLASS)?;
            let counter_obj =
                env.new_object(am_classname!("Counter"), "(J)V", &[i64::from(c).into()])?;
            env.set_field(
                amval,
                "value",
                format!("L{};", am_classname!("Counter")),
                counter_obj.into(),
            )?;
            Ok(amval)
        }
        am::ScalarValue::Timestamp(t) => {
            let t = JValue::Long(*t);
            let date_obj = env.new_object("java/util/Date", "(J)V", &[t]).unwrap();
            let amval = env.alloc_object(TIMESTAMP_CLASS)?;
            env.set_field(amval, "value", "Ljava/util/Date;", date_obj.into())?;
            Ok(amval)
        }
        am::ScalarValue::Unknown { type_code, bytes } => {
            let amval = env.alloc_object(UNKNOWN_CLASS)?;
            let arr = env.byte_array_from_slice(bytes)?;
            env.set_field(
                amval,
                "value",
                "[B",
                unsafe { JObject::from_raw(arr) }.into(),
            )?;
            env.set_field(amval, "typeCode", "I", JValue::Int(*type_code as i32))?;
            Ok(amval)
        }
    }
}
