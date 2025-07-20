use am::transaction::Transactable;
use automerge as am;
use automerge_jni_macros::jni_fn;
use jni::{
    objects::{JObject, JPrimitiveArray, JString},
    sys::{jboolean, jdouble, jlong, jobject, jstring},
};

use crate::{
    expand_mark,
    obj_id::{obj_id_or_throw, JavaObjId},
    AUTOMERGE_EXCEPTION,
};

use super::{do_tx_op, TransactionOp};

struct MarkOp {
    obj: jobject,
    start: usize,
    end: usize,
    name: jstring,
    value: am::ScalarValue,
    expand: jobject,
}

impl TransactionOp for MarkOp {
    type Output = ();

    unsafe fn execute<T: Transactable>(self, env: &mut jni::JNIEnv, tx: &mut T) -> Self::Output {
        let expand_obj = JObject::from_raw(self.expand);
        let expand = expand_mark::from_java(env, expand_obj).unwrap();
        let name_str = JString::from_raw(self.name);
        let name: String = env.get_string(&name_str).unwrap().into();
        let mark = am::marks::Mark::new(name, self.value, self.start, self.end);
        let obj = obj_id_or_throw!(env, self.obj, ());
        match tx.mark(obj, mark, expand) {
            Ok(_) => {}
            Err(e) => {
                let msg = format!("Error marking: {e}");
                env.throw_new(AUTOMERGE_EXCEPTION, msg).unwrap();
            }
        }
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn markUint(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    name_pointer: jni::sys::jstring,
    start: jlong,
    end: jlong,
    value: jlong,
    expand_pointer: jobject,
) {
    let int = match u64::try_from(value) {
        Ok(i) => am::ScalarValue::Uint(i),
        Err(_) => {
            env.throw_new(AUTOMERGE_EXCEPTION, "uint value must not be negative")
                .unwrap();
            return;
        }
    };
    do_tx_op(
        &mut env,
        tx_pointer,
        MarkOp {
            obj: obj_pointer,
            start: start as usize,
            end: end as usize,
            name: name_pointer,
            value: int,
            expand: expand_pointer,
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn markInt(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    name_pointer: jni::sys::jstring,
    start: jlong,
    end: jlong,
    value: jlong,
    expand_pointer: jobject,
) {
    do_tx_op(
        &mut env,
        tx_pointer,
        MarkOp {
            obj: obj_pointer,
            start: start as usize,
            end: end as usize,
            name: name_pointer,
            value: am::ScalarValue::Int(value),
            expand: expand_pointer,
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn markDouble(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    name_pointer: jni::sys::jstring,
    start: jlong,
    end: jlong,
    value: jdouble,
    expand_pointer: jobject,
) {
    do_tx_op(
        &mut env,
        tx_pointer,
        MarkOp {
            obj: obj_pointer,
            start: start as usize,
            end: end as usize,
            name: name_pointer,
            value: am::ScalarValue::F64(value),
            expand: expand_pointer,
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn markBytes(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    name_pointer: jni::sys::jstring,
    start: jlong,
    end: jlong,
    value: jobject,
    expand_pointer: jobject,
) {
    let value = JPrimitiveArray::from_raw(value);
    let bytes = env.convert_byte_array(value).unwrap();
    do_tx_op(
        &mut env,
        tx_pointer,
        MarkOp {
            obj: obj_pointer,
            start: start as usize,
            end: end as usize,
            name: name_pointer,
            value: am::ScalarValue::Bytes(bytes),
            expand: expand_pointer,
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn markString(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    name_pointer: jni::sys::jstring,
    start: jlong,
    end: jlong,
    value: jstring,
    expand_pointer: jobject,
) {
    let value_str = JString::from_raw(value);
    let value: String = env.get_string(&value_str).unwrap().into();
    do_tx_op(
        &mut env,
        tx_pointer,
        MarkOp {
            obj: obj_pointer,
            start: start as usize,
            end: end as usize,
            name: name_pointer,
            value: am::ScalarValue::Str(value.into()),
            expand: expand_pointer,
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn markCounter(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    name_pointer: jni::sys::jstring,
    start: jlong,
    end: jlong,
    value: jlong,
    expand_pointer: jobject,
) {
    do_tx_op(
        &mut env,
        tx_pointer,
        MarkOp {
            obj: obj_pointer,
            start: start as usize,
            end: end as usize,
            name: name_pointer,
            value: am::ScalarValue::Counter(value.into()),
            expand: expand_pointer,
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn markDate(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    name_pointer: jni::sys::jstring,
    start: jlong,
    end: jlong,
    value: jobject,
    expand_pointer: jobject,
) {
    let date = JObject::from_raw(value);
    let date_millis = env
        .call_method(date, "getTime", "()J", &[])
        .unwrap()
        .j()
        .unwrap();
    do_tx_op(
        &mut env,
        tx_pointer,
        MarkOp {
            obj: obj_pointer,
            start: start as usize,
            end: end as usize,
            name: name_pointer,
            value: am::ScalarValue::Timestamp(date_millis),
            expand: expand_pointer,
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn markBool(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    name_pointer: jni::sys::jstring,
    start: jlong,
    end: jlong,
    value: jboolean,
    expand_pointer: jobject,
) {
    let value = value != 0;
    do_tx_op(
        &mut env,
        tx_pointer,
        MarkOp {
            obj: obj_pointer,
            start: start as usize,
            end: end as usize,
            name: name_pointer,
            value: am::ScalarValue::Boolean(value),
            expand: expand_pointer,
        },
    )
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn markNull(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    name_pointer: jni::sys::jstring,
    start: jlong,
    end: jlong,
    expand_pointer: jobject,
) {
    do_tx_op(
        &mut env,
        tx_pointer,
        MarkOp {
            obj: obj_pointer,
            start: start as usize,
            end: end as usize,
            name: name_pointer,
            value: am::ScalarValue::Null,
            expand: expand_pointer,
        },
    )
}

struct Unmark {
    obj: jobject,
    start: usize,
    end: usize,
    name: jstring,
    expand: jobject,
}

impl TransactionOp for Unmark {
    type Output = ();

    unsafe fn execute<T: Transactable>(self, env: &mut jni::JNIEnv, tx: &mut T) -> Self::Output {
        let expand_obj = JObject::from_raw(self.expand);
        let expand = expand_mark::from_java(env, expand_obj).unwrap();
        let name_str = JString::from_raw(self.name);
        let name: String = env.get_string(&name_str).unwrap().into();
        let obj = obj_id_or_throw!(env, self.obj, ());
        match tx.unmark(obj, &name, self.start, self.end, expand) {
            Ok(_) => {}
            Err(e) => {
                let msg = format!("Error marking: {e}");
                env.throw_new(AUTOMERGE_EXCEPTION, msg).unwrap();
            }
        }
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn unMark(
    mut env: jni::JNIEnv,
    _class: jni::objects::JClass,
    tx_pointer: jni::sys::jobject,
    obj_pointer: jni::sys::jobject,
    name_pointer: jni::sys::jstring,
    start: jlong,
    end: jlong,
    expand_pointer: jobject,
) {
    do_tx_op(
        &mut env,
        tx_pointer,
        Unmark {
            obj: obj_pointer,
            start: start as usize,
            end: end as usize,
            name: name_pointer,
            expand: expand_pointer,
        },
    )
}
