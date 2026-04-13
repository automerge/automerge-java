use automerge::{transaction::Transactable, ScalarValue};
use automerge_jni_macros::jni_fn;
use jni::{
    errors::ThrowRuntimeExAndDefault,
    jni_sig, jni_str,
    objects::{JByteArray, JClass, JObject, JString},
    strings::JNIStr,
    sys::jlong,
};

use crate::{
    interop::{throw_amg_exc, unwrap_or_throw_amg_exc},
    JavaObjId,
};

use super::{do_tx_op, TransactionOp};

const UINT_CLASS: &JNIStr = am_classname!("NewValue$UInt");
const INT_CLASS: &JNIStr = am_classname!("NewValue$Int");
const F64_CLASS: &JNIStr = am_classname!("NewValue$F64");
const STR_CLASS: &JNIStr = am_classname!("NewValue$Str");
const BOOL_CLASS: &JNIStr = am_classname!("NewValue$Bool");
const NULL_CLASS: &JNIStr = am_classname!("NewValue$Null");
const BYTES_CLASS: &JNIStr = am_classname!("NewValue$Bytes");
const TIMESTAMP_CLASS: &JNIStr = am_classname!("NewValue$Timestamp");
const COUNTER_CLASS: &JNIStr = am_classname!("NewValue$Counter");

struct SpliceOp {
    obj: JavaObjId,
    index: usize,
    delete: isize,
    values: Vec<ScalarValue>,
}

impl TransactionOp for SpliceOp {
    type Output<'local> = ();

    unsafe fn execute<'local, T: Transactable>(
        self,
        env: &jni::Env<'local>,
        tx: &mut T,
    ) -> Result<Self::Output<'local>, jni::errors::Error> {
        unwrap_or_throw_amg_exc(
            env,
            tx.splice(self.obj, self.index, self.delete, self.values),
        )
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn splice<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    idx: jlong,
    delete: jlong,
    values: JObject<'local>,
) {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj)?;
        let values = JObjToValIter { jiter: values, env }.collect::<Result<Vec<_>, _>>()?;
        do_tx_op(
            env,
            tx,
            SpliceOp {
                obj,
                index: idx as usize,
                delete: delete as isize,
                values,
            },
        )
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

struct JObjToValIter<'a, 'b> {
    jiter: JObject<'a>,
    env: &'b mut jni::Env<'a>,
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
            .call_method(&self.jiter, jni_str!("hasNext"), jni_sig!("()Z"), &[])?
            .z()?;
        if next {
            let obj = self
                .env
                .call_method(
                    &self.jiter,
                    jni_str!("next"),
                    jni_sig!("()Ljava/lang/Object;"),
                    &[],
                )?
                .l()?;
            if self.env.is_instance_of(&obj, INT_CLASS)? {
                let val = self
                    .env
                    .get_field(obj, jni_str!("value"), jni_sig!("J"))?
                    .j()?;
                Ok(Some(ScalarValue::Int(val)))
            } else if self.env.is_instance_of(&obj, UINT_CLASS)? {
                let val = self
                    .env
                    .get_field(obj, jni_str!("value"), jni_sig!("J"))?
                    .j()?;
                // Cast is okay because UInt ensures value is positive
                Ok(Some(ScalarValue::Uint(val as u64)))
            } else if self.env.is_instance_of(&obj, F64_CLASS)? {
                let val = self
                    .env
                    .get_field(obj, jni_str!("value"), jni_sig!("D"))?
                    .d()?;
                Ok(Some(ScalarValue::F64(val)))
            } else if self.env.is_instance_of(&obj, BOOL_CLASS)? {
                let val = self
                    .env
                    .get_field(obj, jni_str!("value"), jni_sig!("Z"))?
                    .z()?;
                Ok(Some(ScalarValue::Boolean(val)))
            } else if self.env.is_instance_of(&obj, BYTES_CLASS)? {
                let bytes = self
                    .env
                    .get_field(obj, jni_str!("value"), jni_sig!("[B"))?
                    .l()?;
                let bytes = JByteArray::cast_local(self.env, bytes)?;
                let val = self.env.convert_byte_array(bytes)?;
                Ok(Some(ScalarValue::Bytes(val)))
            } else if self.env.is_instance_of(&obj, NULL_CLASS)? {
                Ok(Some(ScalarValue::Null))
            } else if self.env.is_instance_of(&obj, STR_CLASS)? {
                let sval = self
                    .env
                    .get_field(obj, jni_str!("value"), jni_sig!("Ljava/lang/String;"))?
                    .l()?;
                let sval = JString::cast_local(self.env, sval)?;
                Ok(Some(ScalarValue::Str(sval.to_string().into())))
            } else if self.env.is_instance_of(&obj, TIMESTAMP_CLASS)? {
                let date = self
                    .env
                    .get_field(obj, jni_str!("value"), jni_sig!("Ljava/util/Date;"))?
                    .l()?;
                let val = self
                    .env
                    .call_method(date, jni_str!("getTime"), jni_sig!("()J"), &[])?
                    .j()?;
                Ok(Some(ScalarValue::Timestamp(val)))
            } else if self.env.is_instance_of(&obj, COUNTER_CLASS)? {
                let val = self
                    .env
                    .get_field(&obj, jni_str!("value"), jni_sig!("J"))?
                    .j()?;
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
    type Item = Result<ScalarValue, jni::errors::Error>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.try_next() {
            Ok(i) => i.map(Ok),
            Err(Error::Jni(e)) => Some(Err(e)),
            Err(Error::InvalidValue) => {
                let _ = throw_amg_exc(self.env, "unsupported type");
                Some(Err(jni::errors::Error::JavaException))
            }
        }
    }
}
