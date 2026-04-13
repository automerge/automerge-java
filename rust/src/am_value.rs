use automerge::{self as am, ObjType};
use jni::{
    jni_sig, jni_str,
    objects::{JObject, JValue},
    signature::RuntimeFieldSignature,
    strings::JNIStr,
};

use crate::{interop::set_array_field, JavaObjId};

const FLOAT_CLASS: &JNIStr = am_classname!("AmValue$F64");
const BYTE_CLASS: &JNIStr = am_classname!("AmValue$Bytes");
const STR_CLASS: &JNIStr = am_classname!("AmValue$Str");
const INT_CLASS: &JNIStr = am_classname!("AmValue$Int");
const UINT_CLASS: &JNIStr = am_classname!("AmValue$UInt");
const BOOL_CLASS: &JNIStr = am_classname!("AmValue$Bool");
const NULL_CLASS: &JNIStr = am_classname!("AmValue$Null");
const COUNTER_CLASS: &JNIStr = am_classname!("AmValue$Counter");
const TIMESTAMP_CLASS: &JNIStr = am_classname!("AmValue$Timestamp");
const UNKNOWN_CLASS: &JNIStr = am_classname!("AmValue$Unknown");
const MAP_CLASS: &JNIStr = am_classname!("AmValue$Map");
const LIST_CLASS: &JNIStr = am_classname!("AmValue$List");
const TEXT_CLASS: &JNIStr = am_classname!("AmValue$Text");

pub(crate) unsafe fn to_optional_amvalue<'local>(
    env: &mut jni::Env<'local>,
    val: Option<(automerge::Value, automerge::ObjId)>,
) -> Result<JObject<'local>, jni::errors::Error> {
    match val {
        Some(val) => {
            let val = to_amvalue(env, val)?;
            let opt = env
                .call_static_method(
                    jni_str!("java/util/Optional"),
                    jni_str!("of"),
                    jni_sig!("(Ljava/lang/Object;)Ljava/util/Optional;"),
                    &[(&val).into()],
                )?
                .l()?;
            Ok(opt)
        }
        None => {
            let opt = env
                .call_static_method(
                    jni_str!("java/util/Optional"),
                    jni_str!("empty"),
                    jni_sig!("()Ljava/util/Optional;"),
                    &[],
                )?
                .l()?;
            Ok(opt)
        }
    }
}

pub(crate) unsafe fn to_amvalue<'local>(
    env: &mut jni::Env<'local>,
    val: (automerge::Value, automerge::ObjId),
) -> Result<JObject<'local>, jni::errors::Error> {
    match val {
        (automerge::Value::Scalar(s), _) => scalar_to_amvalue(env, s.as_ref()),
        (automerge::Value::Object(objtype), oid) => {
            let oid = JavaObjId::from(oid).into_jobject(env)?;
            // TODO: replace with jni_sig!
            let field_sig =
                RuntimeFieldSignature::from_str(format!("L{};", am_classname!("ObjectId")))
                    .unwrap();
            match objtype {
                ObjType::Map | ObjType::Table => {
                    let amval = env.alloc_object(MAP_CLASS)?;
                    env.set_field(
                        &amval,
                        jni_str!("id"),
                        field_sig.field_signature(),
                        (&oid).into(),
                    )?;
                    Ok(amval)
                }
                ObjType::List => {
                    let amval = env.alloc_object(LIST_CLASS)?;
                    env.set_field(
                        &amval,
                        jni_str!("id"),
                        field_sig.field_signature(),
                        (&oid).into(),
                    )?;
                    Ok(amval)
                }
                ObjType::Text => {
                    let amval = env.alloc_object(TEXT_CLASS)?;
                    env.set_field(
                        &amval,
                        jni_str!("id"),
                        field_sig.field_signature(),
                        (&oid).into(),
                    )?;
                    Ok(amval)
                }
            }
        }
    }
}

pub(crate) fn scalar_to_amvalue<'local>(
    env: &mut jni::Env<'local>,
    val: &am::ScalarValue,
) -> jni::errors::Result<JObject<'local>> {
    match val {
        am::ScalarValue::F64(v) => {
            let amval = env.alloc_object(FLOAT_CLASS)?;
            env.set_field(&amval, jni_str!("value"), jni_sig!("D"), JValue::Double(*v))?;
            Ok(amval)
        }
        am::ScalarValue::Bytes(v) => {
            let amval = env.alloc_object(BYTE_CLASS)?;
            let arr = env.byte_array_from_slice(v)?;
            unsafe {
                set_array_field(
                    env,
                    &amval,
                    jni_str!("value"),
                    jni_sig!("[B"),
                    (&arr).into(),
                )?
            };
            Ok(amval)
        }
        am::ScalarValue::Str(s) => {
            let s = env.new_string(s.as_str())?;
            let amval = env.alloc_object(STR_CLASS)?;
            env.set_field(
                &amval,
                jni_str!("value"),
                jni_sig!("Ljava/lang/String;"),
                (&s).into(),
            )?;
            Ok(amval)
        }
        am::ScalarValue::Int(i) => {
            let amval = env.alloc_object(INT_CLASS)?;
            env.set_field(&amval, jni_str!("value"), jni_sig!("J"), JValue::Long(*i))?;
            Ok(amval)
        }
        am::ScalarValue::Uint(i) => {
            let amval = env.alloc_object(UINT_CLASS)?;
            env.set_field(
                &amval,
                jni_str!("value"),
                jni_sig!("J"),
                JValue::Long(*i as i64),
            )?;
            Ok(amval)
        }
        am::ScalarValue::Boolean(val) => {
            let amval = env.alloc_object(BOOL_CLASS)?;
            env.set_field(&amval, jni_str!("value"), jni_sig!("Z"), JValue::Bool(*val))?;
            Ok(amval)
        }
        am::ScalarValue::Null => {
            let amval = env.alloc_object(NULL_CLASS)?;
            Ok(amval)
        }
        am::ScalarValue::Counter(c) => {
            let amval = env.alloc_object(COUNTER_CLASS)?;
            let counter_obj = env.new_object(
                am_classname!("Counter"),
                jni_sig!("(J)V"),
                &[i64::from(c).into()],
            )?;
            // TODO: replace with jni_sig
            let field_sig =
                RuntimeFieldSignature::from_str(format!("L{};", am_classname!("Counter"))).unwrap();

            env.set_field(
                &amval,
                jni_str!("value"),
                field_sig.field_signature(),
                (&counter_obj).into(),
            )?;
            Ok(amval)
        }
        am::ScalarValue::Timestamp(t) => {
            let t = JValue::Long(*t);
            let date_obj = env.new_object(jni_str!("java/util/Date"), jni_sig!("(J)V"), &[t])?;
            let amval = env.alloc_object(TIMESTAMP_CLASS)?;
            env.set_field(
                &amval,
                jni_str!("value"),
                jni_sig!("Ljava/util/Date;"),
                (&date_obj).into(),
            )?;
            Ok(amval)
        }
        am::ScalarValue::Unknown { type_code, bytes } => {
            let amval = env.alloc_object(UNKNOWN_CLASS)?;
            let arr = env.byte_array_from_slice(bytes)?;
            unsafe {
                set_array_field(
                    env,
                    &amval,
                    jni_str!("value"),
                    jni_sig!("[B"),
                    (&arr).into(),
                )?
            };
            env.set_field(
                &amval,
                jni_str!("typeCode"),
                jni_sig!("I"),
                JValue::Int(*type_code as i32),
            )?;
            Ok(amval)
        }
    }
}
