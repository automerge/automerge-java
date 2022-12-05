use automerge as am;
use automerge::transaction::Transactable;
use automerge_jni_macros::jni_fn;
use jni::sys::jboolean;
use jni::{
    objects::{JObject, JString},
    sys::{jbyteArray, jint, jlong, jobject},
};

use crate::obj_type::JavaObjType;
use crate::prop::JProp;
use crate::{obj_id::JavaObjId, AUTOMERGE_EXCEPTION};

use super::{do_tx_op, TransactionOp};

struct SetOp<'a, V: Into<automerge::ScalarValue>> {
    obj: jobject,
    prop: JProp<'a>,
    value: V,
}

impl<'a, V: Into<automerge::ScalarValue>> TransactionOp for SetOp<'a, V> {
    type Output = ();

    unsafe fn execute<T: Transactable>(self, env: jni::JNIEnv, tx: &mut T) -> Self::Output {
        let key = match self.prop.try_into_prop(env) {
            Ok(k) => k,
            Err(e) => {
                env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
                return;
            }
        };
        let obj = JavaObjId::from_raw(&env, self.obj).unwrap();

        match tx.put(obj, key, self.value) {
            Ok(_) => {}
            Err(e) => {
                env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
            }
        }
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setDoubleInMap(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    key: JString,
    value: jni::sys::jdouble,
) {
    do_tx_op(
        env,
        tx_pointer,
        SetOp {
            obj: obj_pointer,
            prop: key.into(),
            value,
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setBytesInMap(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    key: JString,
    value: jbyteArray,
) {
    let bytes = env.convert_byte_array(value).unwrap();
    do_tx_op(
        env,
        tx_pointer,
        SetOp {
            obj: obj_pointer,
            prop: key.into(),
            value: bytes.as_slice().to_vec(),
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setStringInMap(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    key: JString,
    value: JString,
) {
    let val: String = env.get_string(value).unwrap().into();
    do_tx_op(
        env,
        tx_pointer,
        SetOp {
            obj: obj_pointer,
            prop: key.into(),
            value: val,
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setIntInMap(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    key: JString,
    value: jint,
) {
    do_tx_op(
        env,
        tx_pointer,
        SetOp {
            obj: obj_pointer,
            prop: key.into(),
            value: value as i64,
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setUintInMap(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    key: JString,
    value: jint,
) {
    do_tx_op(
        env,
        tx_pointer,
        SetOp {
            obj: obj_pointer,
            prop: key.into(),
            value: am::ScalarValue::Uint(value as u64),
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setBoolInMap(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    key: JString,
    value: jboolean,
) {
    do_tx_op(
        env,
        tx_pointer,
        SetOp {
            obj: obj_pointer,
            prop: key.into(),
            value: am::ScalarValue::Boolean(value != 0),
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setNullInMap(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    key: JString,
) {
    do_tx_op(
        env,
        tx_pointer,
        SetOp {
            obj: obj_pointer,
            prop: key.into(),
            value: am::ScalarValue::Null,
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setCounterInMap(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    key: JString,
    value: jlong,
) {
    do_tx_op(
        env,
        tx_pointer,
        SetOp {
            obj: obj_pointer,
            prop: key.into(),
            value: am::ScalarValue::counter(value),
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setDateInMap(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    key: JString,
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
        SetOp {
            obj: obj_pointer,
            prop: key.into(),
            value: am::ScalarValue::Timestamp(date_millis),
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setDoubleInList(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    idx: jlong,
    value: jni::sys::jdouble,
) {
    do_tx_op(
        env,
        tx_pointer,
        SetOp {
            obj: obj_pointer,
            prop: idx.into(),
            value,
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setIntInList(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    idx: jlong,
    value: jint,
) {
    do_tx_op(
        env,
        tx_pointer,
        SetOp {
            obj: obj_pointer,
            prop: idx.into(),
            value: value as i64,
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setUintInList(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    idx: jlong,
    value: jint,
) {
    do_tx_op(
        env,
        tx_pointer,
        SetOp {
            obj: obj_pointer,
            prop: idx.into(),
            value: am::ScalarValue::Uint(value as u64),
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setStringInList(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    idx: jlong,
    value: JString,
) {
    let val: String = env.get_string(value).unwrap().into();
    do_tx_op(
        env,
        tx_pointer,
        SetOp {
            obj: obj_pointer,
            prop: idx.into(),
            value: val,
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setBytesInList(
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
        SetOp {
            obj: obj_pointer,
            prop: idx.into(),
            value: bytes,
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setBoolInList(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    idx: jlong,
    value: jboolean,
) {
    do_tx_op(
        env,
        tx_pointer,
        SetOp {
            obj: obj_pointer,
            prop: idx.into(),
            value: am::ScalarValue::Boolean(value == 1),
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setDateInList(
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
        SetOp {
            obj: obj_pointer,
            prop: idx.into(),
            value: am::ScalarValue::Timestamp(date_millis),
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setCounterInList(
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
        SetOp {
            obj: obj_pointer,
            prop: idx.into(),
            value: am::ScalarValue::counter(value),
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setNullInList(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    idx: jlong,
) {
    do_tx_op(
        env,
        tx_pointer,
        SetOp {
            obj: obj_pointer,
            prop: idx.into(),
            value: am::ScalarValue::Null,
        },
    )
}

struct SetObjOp {
    obj: jobject,
    key: automerge::Prop,
    value: am::ObjType,
}

impl TransactionOp for SetObjOp {
    type Output = jobject;

    unsafe fn execute<T: Transactable>(self, env: jni::JNIEnv, tx: &mut T) -> Self::Output {
        let obj = JavaObjId::from_raw(&env, self.obj).unwrap();
        let oid = match tx.put_object(obj, self.key, self.value) {
            Ok(oid) => oid,
            Err(e) => {
                env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
                return JObject::null().into_raw();
            }
        };
        let jobjid = JavaObjId::from(oid);
        jobjid.into_raw(&env).unwrap()
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setObjectInMap(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    key: JString,
    value: jobject,
) -> jobject {
    let obj_type = JavaObjType::from_java_enum(&env, value).unwrap();
    let jstr = env.get_string(key).unwrap();
    let key = jstr.to_str().unwrap();
    do_tx_op(
        env,
        tx_pointer,
        SetObjOp {
            obj: obj_pointer,
            key: key.into(),
            value: obj_type.into(),
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn setObjectInList(
    env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    idx: jlong,
    value: jobject,
) -> jobject {
    let obj_type = JavaObjType::from_java_enum(&env, value).unwrap();
    let idx = match usize::try_from(idx) {
        Ok(idx) => idx,
        Err(_) => {
            env.throw_new(AUTOMERGE_EXCEPTION, "index must be non-negative")
                .unwrap();
            return JObject::null().into_raw();
        }
    };
    do_tx_op(
        env,
        tx_pointer,
        SetObjOp {
            obj: obj_pointer,
            key: idx.into(),
            value: obj_type.into(),
        },
    )
}
