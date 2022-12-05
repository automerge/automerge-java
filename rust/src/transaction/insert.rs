use am::ObjType;
use automerge as am;
use automerge_jni_macros::jni_fn;
use jni::{
    objects::{JObject, JString},
    sys::{jboolean, jbyteArray, jdouble, jlong, jobject},
};

use crate::{obj_id::JavaObjId, obj_type::JavaObjType, AUTOMERGE_EXCEPTION};

use super::{do_tx_op, TransactionOp};

struct InsertOp<V> {
    obj: jobject,
    index: jlong,
    value: V,
}

impl TransactionOp for InsertOp<am::ScalarValue> {
    type Output = ();
    unsafe fn execute<T: am::transaction::Transactable>(
        self,
        env: jni::JNIEnv,
        tx: &mut T,
    ) -> Self::Output {
        let obj = JavaObjId::from_raw(&env, self.obj).unwrap();
        let idx = match usize::try_from(self.index) {
            Ok(i) => i,
            Err(_) => {
                env.throw_new(AUTOMERGE_EXCEPTION, "index cannot be negative")
                    .unwrap();
                return;
            }
        };
        match tx.insert(obj, idx, self.value) {
            Ok(_) => {}
            Err(e) => {
                env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
            }
        }
    }
}

impl TransactionOp for InsertOp<ObjType> {
    type Output = jobject;

    unsafe fn execute<T: am::transaction::Transactable>(
        self,
        env: jni::JNIEnv,
        tx: &mut T,
    ) -> Self::Output {
        let obj = JavaObjId::from_raw(&env, self.obj).unwrap();
        let idx = match usize::try_from(self.index) {
            Ok(i) => i,
            Err(_) => {
                env.throw_new(AUTOMERGE_EXCEPTION, "index cannot be negative")
                    .unwrap();
                return JObject::null().into_raw();
            }
        };
        let value = match tx.insert_object(obj, idx, self.value) {
            Ok(v) => v,
            Err(e) => {
                env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
                return JObject::null().into_raw();
            }
        };
        JavaObjId::from(value).into_raw(&env).unwrap()
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn insertDoubleInList(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    idx: jlong,
    value: jdouble,
) {
    do_tx_op(
        env,
        tx_pointer,
        InsertOp {
            obj: obj_pointer,
            index: idx,
            value: am::ScalarValue::F64(value),
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn insertStringInList(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    idx: jlong,
    value: JString,
) {
    let value: String = env.get_string(value).unwrap().into();
    do_tx_op(
        env,
        tx_pointer,
        InsertOp {
            obj: obj_pointer,
            index: idx,
            value: am::ScalarValue::Str(value.into()),
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn insertIntInList(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    idx: jlong,
    value: jlong,
) {
    do_tx_op(
        env,
        tx_pointer,
        InsertOp {
            obj: obj_pointer,
            index: idx,
            value: am::ScalarValue::Int(value),
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn insertUintInList(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    idx: jlong,
    value: jlong,
) {
    let int = match u64::try_from(value) {
        Ok(i) => i,
        Err(_) => {
            env.throw_new(AUTOMERGE_EXCEPTION, "uint value must not be negative")
                .unwrap();
            return;
        }
    };
    do_tx_op(
        env,
        tx_pointer,
        InsertOp {
            obj: obj_pointer,
            index: idx,
            value: am::ScalarValue::Uint(int),
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn insertBytesInList(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    idx: jlong,
    value: jbyteArray,
) {
    let bytes = env.convert_byte_array(value).unwrap();
    do_tx_op(
        env,
        tx_pointer,
        InsertOp {
            obj: obj_pointer,
            index: idx,
            value: am::ScalarValue::Bytes(bytes),
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn insertNullInList(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    idx: jlong,
) {
    do_tx_op(
        env,
        tx_pointer,
        InsertOp {
            obj: obj_pointer,
            index: idx,
            value: am::ScalarValue::Null,
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn insertCounterInList(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    idx: jlong,
    value: jlong,
) {
    do_tx_op(
        env,
        tx_pointer,
        InsertOp {
            obj: obj_pointer,
            index: idx,
            value: am::ScalarValue::counter(value),
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn insertDateInList(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    idx: jlong,
    value: jobject,
) {
    let date = JObject::from_raw(value);
    let date_millis = env
        .call_method(date, "getTime", "()J", &[])
        .unwrap()
        .j()
        .unwrap();
    do_tx_op(
        env,
        tx_pointer,
        InsertOp {
            obj: obj_pointer,
            index: idx,
            value: am::ScalarValue::Timestamp(date_millis),
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn insertBoolInList(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    idx: jlong,
    value: jboolean,
) {
    let value = value != 0;
    do_tx_op(
        env,
        tx_pointer,
        InsertOp {
            obj: obj_pointer,
            index: idx,
            value: am::ScalarValue::Boolean(value),
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn insertObjectInList(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    idx: jlong,
    value: jobject,
) -> jobject {
    let obj_type = JavaObjType::from_java_enum(&env, value).unwrap();
    do_tx_op(
        env,
        tx_pointer,
        InsertOp {
            obj: obj_pointer,
            index: idx,
            value: am::ObjType::from(obj_type),
        },
    )
}
