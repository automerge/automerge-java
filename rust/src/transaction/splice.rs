use automerge::{transaction::Transactable, ScalarValue};
use automerge_jni_macros::jni_fn;
use jni::{
    objects::{JObject, JPrimitiveArray},
    sys::{jlong, jobject},
};

use crate::{obj_id::obj_id_or_throw, JavaObjId, AUTOMERGE_EXCEPTION};

use super::{do_tx_op, TransactionOp};

const UINT_CLASS: &str = am_classname!("NewValue$UInt");
const INT_CLASS: &str = am_classname!("NewValue$Int");
const F64_CLASS: &str = am_classname!("NewValue$F64");
const STR_CLASS: &str = am_classname!("NewValue$Str");
const BOOL_CLASS: &str = am_classname!("NewValue$Bool");
const NULL_CLASS: &str = am_classname!("NewValue$Null");
const BYTES_CLASS: &str = am_classname!("NewValue$Bytes");
const TIMESTAMP_CLASS: &str = am_classname!("NewValue$Timestamp");
const COUNTER_CLASS: &str = am_classname!("NewValue$Counter");

struct SpliceOp {
    obj: jobject,
    index: usize,
    delete: isize,
    values: jobject,
}

impl TransactionOp for SpliceOp {
    type Output = ();

    unsafe fn execute<T: Transactable>(self, env: &mut jni::JNIEnv, tx: &mut T) -> Self::Output {
        let obj = obj_id_or_throw!(env, self.obj, ());
        let iter = JObjToValIter {
            jiter: JObject::from_raw(self.values),
            env,
        };
        match tx.splice(obj, self.index, self.delete, iter) {
            Ok(_) => {}
            Err(e) => {
                env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
            }
        }
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn splice(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    idx: jlong,
    delete: jlong,
    values: jobject,
) {
    do_tx_op(
        &mut env,
        tx_pointer,
        SpliceOp {
            obj: obj_pointer,
            index: idx as usize,
            delete: delete as isize,
            values,
        },
    )
}

struct JObjToValIter<'a, 'b> {
    jiter: JObject<'a>,
    env: &'b mut jni::JNIEnv<'a>,
}

#[derive(Debug, thiserror::Error)]
enum Error {
    #[error(transparent)]
    Jni(#[from] jni::errors::Error),
    #[error("invalid value")]
    InvalidValue,
}

impl<'a, 'b> JObjToValIter<'a, 'b> {
    fn try_next(&mut self) -> Result<Option<ScalarValue>, Error> {
        let next = self
            .env
            .call_method(&self.jiter, "hasNext", "()Z", &[])?
            .z()?;
        if next {
            let obj = self
                .env
                .call_method(&self.jiter, "next", "()Ljava/lang/Object;", &[])?
                .l()?;
            if self.env.is_instance_of(&obj, INT_CLASS)? {
                let val = self.env.get_field(obj, "value", "J")?.j()?;
                Ok(Some(ScalarValue::Int(val)))
            } else if self.env.is_instance_of(&obj, UINT_CLASS)? {
                let val = self.env.get_field(obj, "value", "J")?.j()?;
                // Cast is okay because UInt ensures value is positive
                Ok(Some(ScalarValue::Uint(val as u64)))
            } else if self.env.is_instance_of(&obj, F64_CLASS)? {
                let val = self.env.get_field(obj, "value", "D")?.d()?;
                Ok(Some(ScalarValue::F64(val)))
            } else if self.env.is_instance_of(&obj, BOOL_CLASS)? {
                let val = self.env.get_field(obj, "value", "Z")?.z()?;
                Ok(Some(ScalarValue::Boolean(val)))
            } else if self.env.is_instance_of(&obj, BYTES_CLASS)? {
                let bytes = self.env.get_field(obj, "value", "[B")?.l()?;
                let bytes = JPrimitiveArray::from(bytes);
                let val = self.env.convert_byte_array(&bytes)?;
                Ok(Some(ScalarValue::Bytes(val)))
            } else if self.env.is_instance_of(&obj, NULL_CLASS)? {
                Ok(Some(ScalarValue::Null))
            } else if self.env.is_instance_of(&obj, STR_CLASS)? {
                let sval = self
                    .env
                    .get_field(obj, "value", "Ljava/lang/String;")?
                    .l()?;
                let s = self.env.get_string((&sval).into())?;
                let sref = s.to_str();
                Ok(Some(ScalarValue::Str(sref.unwrap().to_string().into())))
            } else if self.env.is_instance_of(&obj, TIMESTAMP_CLASS)? {
                let date = self.env.get_field(obj, "value", "Ljava/util/Date;")?.l()?;
                let val = self.env.call_method(date, "getTime", "()J", &[])?.j()?;
                Ok(Some(ScalarValue::Timestamp(val)))
            } else if self.env.is_instance_of(&obj, COUNTER_CLASS)? {
                let val = self.env.get_field(&obj, "value", "J")?.j()?;
                Ok(Some(ScalarValue::Counter(val.into())))
            } else {
                //self.env.throw_new(AUTOMERGE_EXCEPTION, "Unsupported type")?;
                Err(Error::InvalidValue)
            }
        } else {
            Ok(None)
        }
    }
}

impl<'a, 'b> Iterator for JObjToValIter<'a, 'b> {
    type Item = ScalarValue;
    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(i) => i,
            Err(Error::Jni(e)) => panic!("Jni error: {e}"),
            Err(Error::InvalidValue) => {
                self.env
                    .throw_new(AUTOMERGE_EXCEPTION, "Unsupported type")
                    .unwrap();
                None
            }
        }
    }
}
