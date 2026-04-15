use automerge::{self as am, ObjType};
use jni::{jni_sig, jni_str, objects::JValue};

use crate::{
    bindings::{
        AmValue, AmValueBool, AmValueBytes, AmValueCounter, AmValueF64, AmValueInt, AmValueList,
        AmValueMap, AmValueNull, AmValueStr, AmValueText, AmValueTimestamp, AmValueUInt,
        AmValueUnknown, JavaCounter, Optional,
    },
    JavaObjId,
};

pub(crate) unsafe fn to_optional_amvalue<'local>(
    env: &mut jni::Env<'local>,
    val: Option<(automerge::Value, automerge::ObjId)>,
) -> Result<Optional<'local>, jni::errors::Error> {
    match val {
        Some(val) => {
            let val = to_amvalue(env, val)?;
            Optional::of(env, val)
        }
        None => Optional::empty(env),
    }
}

pub(crate) unsafe fn to_amvalue<'local>(
    env: &mut jni::Env<'local>,
    val: (automerge::Value, automerge::ObjId),
) -> Result<AmValue<'local>, jni::errors::Error> {
    match val {
        (automerge::Value::Scalar(s), _) => scalar_to_amvalue(env, s.as_ref()),
        (automerge::Value::Object(objtype), oid) => {
            let oid = JavaObjId::from(oid).into_object_id(env)?;
            match objtype {
                ObjType::Map | ObjType::Table => {
                    let amval = AmValueMap::new(env)?;
                    amval.set_id(env, &oid)?;
                    Ok(amval.into())
                }
                ObjType::List => {
                    let amval = AmValueList::new(env)?;
                    amval.set_id(env, &oid)?;
                    Ok(amval.into())
                }
                ObjType::Text => {
                    let amval = AmValueText::new(env)?;
                    amval.set_id(env, &oid)?;
                    Ok(amval.into())
                }
            }
        }
    }
}

pub(crate) fn scalar_to_amvalue<'local>(
    env: &mut jni::Env<'local>,
    val: &am::ScalarValue,
) -> jni::errors::Result<AmValue<'local>> {
    let obj: AmValue<'local> = match val {
        am::ScalarValue::F64(v) => {
            let amval = AmValueF64::new(env)?;
            amval.set_value(env, *v)?;
            amval.into()
        }
        am::ScalarValue::Bytes(v) => {
            let amval = AmValueBytes::new(env)?;
            let arr = env.byte_array_from_slice(v)?;
            amval.set_value(env, &arr)?;
            amval.into()
        }
        am::ScalarValue::Str(s) => {
            let jstr = env.new_string(s.as_str())?;
            let amval = AmValueStr::new(env)?;
            amval.set_value(env, &jstr)?;
            amval.into()
        }
        am::ScalarValue::Int(i) => {
            let amval = AmValueInt::new(env)?;
            amval.set_value(env, *i)?;
            amval.into()
        }
        am::ScalarValue::Uint(i) => {
            let amval = AmValueUInt::new(env)?;
            amval.set_value(env, *i as i64)?;
            amval.into()
        }
        am::ScalarValue::Boolean(v) => {
            let amval = AmValueBool::new(env)?;
            amval.set_value(env, *v)?;
            amval.into()
        }
        am::ScalarValue::Null => AmValueNull::new(env)?.into(),
        am::ScalarValue::Counter(c) => {
            let counter = JavaCounter::new(env, i64::from(c))?;
            let amval = AmValueCounter::new(env)?;
            amval.set_value(env, &counter)?;
            amval.into()
        }
        am::ScalarValue::Timestamp(t) => {
            let date_obj = env.new_object(
                jni_str!("java/util/Date"),
                jni_sig!("(J)V"),
                &[JValue::Long(*t)],
            )?;
            let amval = AmValueTimestamp::new(env)?;
            amval.set_value(env, &date_obj)?;
            amval.into()
        }
        am::ScalarValue::Unknown { type_code, bytes } => {
            let amval = AmValueUnknown::new(env)?;
            let arr = env.byte_array_from_slice(bytes)?;
            amval.set_value(env, &arr)?;
            amval.set_type_code(env, *type_code as i32)?;
            amval.into()
        }
    };
    Ok(obj)
}
