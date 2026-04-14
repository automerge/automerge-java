use automerge::{transaction::Transactable, ScalarValue};
use jni::{
    jni_sig, jni_str,
    objects::{JClass, JIterator, JObject},
    refs::Reference,
    sys::jlong,
    NativeMethod,
};

use crate::bindings;
use crate::bindings::{
    NewValueBool, NewValueBytes, NewValueCounter, NewValueF64, NewValueInt, NewValueNull,
    NewValueStr, NewValueTimestamp, NewValueUInt,
};
use crate::{
    interop::{throw_amg_exc, unwrap_or_throw_amg_exc},
    JavaObjId,
};

use super::{do_tx_op, TransactionOp};

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

const _METHODS: &[NativeMethod] = &[
    ams_native! { static extern fn splice(tx: bindings::TransactionPointer, obj: bindings::ObjectId, idx: jlong, delete: jlong, values: java.util.Iterator) },
];

fn splice<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    idx: jlong,
    delete: jlong,
    values: JIterator<'local>,
) -> jni::errors::Result<()> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    let values = JObjToValIter {
        jiter: values.into(),
        env,
    }
    .collect::<Result<Vec<_>, _>>()?;
    unsafe {
        do_tx_op(
            env,
            tx.into(),
            SpliceOp {
                obj,
                index: idx as usize,
                delete: delete as isize,
                values,
            },
        )
    }
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
        if !next {
            return Ok(None);
        }
        let obj = self
            .env
            .call_method(
                &self.jiter,
                jni_str!("next"),
                jni_sig!("()Ljava/lang/Object;"),
                &[],
            )?
            .l()?;
        let env = &mut *self.env;
        if env.is_instance_of(&obj, NewValueInt::class_name().as_ref())? {
            let v = NewValueInt::cast_local(env, obj)?;
            Ok(Some(ScalarValue::Int(v.value(env)?)))
        } else if env.is_instance_of(&obj, NewValueUInt::class_name().as_ref())? {
            let v = NewValueUInt::cast_local(env, obj)?;
            Ok(Some(ScalarValue::Uint(v.value(env)? as u64)))
        } else if env.is_instance_of(&obj, NewValueF64::class_name().as_ref())? {
            let v = NewValueF64::cast_local(env, obj)?;
            Ok(Some(ScalarValue::F64(v.value(env)?)))
        } else if env.is_instance_of(&obj, NewValueBool::class_name().as_ref())? {
            let v = NewValueBool::cast_local(env, obj)?;
            Ok(Some(ScalarValue::Boolean(v.value(env)?)))
        } else if env.is_instance_of(&obj, NewValueBytes::class_name().as_ref())? {
            let v = NewValueBytes::cast_local(env, obj)?;
            let bytes = v.value(env)?;
            Ok(Some(ScalarValue::Bytes(env.convert_byte_array(&bytes)?)))
        } else if env.is_instance_of(&obj, NewValueNull::class_name().as_ref())? {
            Ok(Some(ScalarValue::Null))
        } else if env.is_instance_of(&obj, NewValueStr::class_name().as_ref())? {
            let v = NewValueStr::cast_local(env, obj)?;
            let sval = v.value(env)?;
            Ok(Some(ScalarValue::Str(sval.to_string().into())))
        } else if env.is_instance_of(&obj, NewValueTimestamp::class_name().as_ref())? {
            let v = NewValueTimestamp::cast_local(env, obj)?;
            let date = v.value(env)?;
            let millis = env
                .call_method(date, jni_str!("getTime"), jni_sig!("()J"), &[])?
                .j()?;
            Ok(Some(ScalarValue::Timestamp(millis)))
        } else if env.is_instance_of(&obj, NewValueCounter::class_name().as_ref())? {
            let v = NewValueCounter::cast_local(env, obj)?;
            Ok(Some(ScalarValue::Counter(v.value(env)?.into())))
        } else {
            Err(Error::InvalidValue)
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
