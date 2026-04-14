use am::ObjType;
use automerge as am;
use jni::{
    jni_sig, jni_str,
    objects::{JByteArray, JClass, JObject, JString},
    sys::{jboolean, jdouble, jlong},
    NativeMethod,
};

use crate::{
    interop::{read_u64, read_usize, unwrap_or_throw_amg_exc},
    obj_id::JavaObjId,
    obj_type::JavaObjType,
};

use super::{do_tx_op, TransactionOp};

struct InsertOp<V> {
    obj: JavaObjId,
    index: jlong,
    value: V,
}

impl TransactionOp for InsertOp<am::ScalarValue> {
    type Output<'b> = ();
    unsafe fn execute<'a, T: am::transaction::Transactable>(
        self,
        env: &jni::Env<'a>,
        tx: &mut T,
    ) -> Result<Self::Output<'a>, jni::errors::Error> {
        let idx = read_usize(env, self.index)?;
        unwrap_or_throw_amg_exc(env, tx.insert(self.obj, idx, self.value))
    }
}

impl TransactionOp for InsertOp<ObjType> {
    type Output<'b> = JavaObjId;

    unsafe fn execute<'a, T: am::transaction::Transactable>(
        self,
        env: &jni::Env<'a>,
        tx: &mut T,
    ) -> Result<Self::Output<'a>, jni::errors::Error> {
        let idx = read_usize(env, self.index)?;
        let value = unwrap_or_throw_amg_exc(env, tx.insert_object(self.obj, idx, self.value))?;
        Ok(JavaObjId::from(value))
    }
}

use crate::bindings;

const _METHODS: &[NativeMethod] = &[
    ams_native! { static extern fn insert_double_in_list(tx: bindings::TransactionPointer, obj: bindings::ObjectId, idx: jlong, value: jdouble) },
    ams_native! { static extern fn insert_string_in_list(tx: bindings::TransactionPointer, obj: bindings::ObjectId, idx: jlong, value: JString) },
    ams_native! { static extern fn insert_int_in_list(tx: bindings::TransactionPointer, obj: bindings::ObjectId, idx: jlong, value: jlong) },
    ams_native! { static extern fn insert_uint_in_list(tx: bindings::TransactionPointer, obj: bindings::ObjectId, idx: jlong, value: jlong) },
    ams_native! { static extern fn insert_bytes_in_list(tx: bindings::TransactionPointer, obj: bindings::ObjectId, idx: jlong, value: jbyte[]) },
    ams_native! { static extern fn insert_null_in_list(tx: bindings::TransactionPointer, obj: bindings::ObjectId, idx: jlong) },
    ams_native! { static extern fn insert_counter_in_list(tx: bindings::TransactionPointer, obj: bindings::ObjectId, idx: jlong, value: jlong) },
    ams_native! { static extern fn insert_date_in_list(tx: bindings::TransactionPointer, obj: bindings::ObjectId, idx: jlong, date: java.util.Date) },
    ams_native! { static extern fn insert_bool_in_list(tx: bindings::TransactionPointer, obj: bindings::ObjectId, idx: jlong, value: jboolean) },
    ams_native! { static extern fn insert_object_in_list(tx: bindings::TransactionPointer, obj: bindings::ObjectId, idx: jlong, value: org.automerge.ObjectType) -> bindings::ObjectId },
];

fn insert_double_in_list<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    idx: jlong,
    value: jdouble,
) -> jni::errors::Result<()> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    unsafe {
        do_tx_op(
            env,
            tx.into(),
            InsertOp {
                obj,
                index: idx,
                value: am::ScalarValue::F64(value),
            },
        )
    }
}

fn insert_string_in_list<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    idx: jlong,
    value: JString<'local>,
) -> jni::errors::Result<()> {
    let value: String = value.to_string();
    let obj = JavaObjId::from_object_id(env, obj)?;
    unsafe {
        do_tx_op(
            env,
            tx.into(),
            InsertOp {
                obj,
                index: idx,
                value: am::ScalarValue::Str(value.into()),
            },
        )
    }
}

fn insert_int_in_list<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    idx: jlong,
    value: jlong,
) -> jni::errors::Result<()> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    unsafe {
        do_tx_op(
            env,
            tx.into(),
            InsertOp {
                obj,
                index: idx,
                value: am::ScalarValue::Int(value),
            },
        )
    }
}

fn insert_uint_in_list<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    idx: jlong,
    value: jlong,
) -> jni::errors::Result<()> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    let int = read_u64(env, value)?;
    unsafe {
        do_tx_op(
            env,
            tx.into(),
            InsertOp {
                obj,
                index: idx,
                value: am::ScalarValue::Uint(int),
            },
        )
    }
}

fn insert_bytes_in_list<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    idx: jlong,
    value: JByteArray<'local>,
) -> jni::errors::Result<()> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    let bytes = env.convert_byte_array(&value)?;
    unsafe {
        do_tx_op(
            env,
            tx.into(),
            InsertOp {
                obj,
                index: idx,
                value: am::ScalarValue::Bytes(bytes),
            },
        )
    }
}

fn insert_null_in_list<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    idx: jlong,
) -> jni::errors::Result<()> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    unsafe {
        do_tx_op(
            env,
            tx.into(),
            InsertOp {
                obj,
                index: idx,
                value: am::ScalarValue::Null,
            },
        )
    }
}

fn insert_counter_in_list<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    idx: jlong,
    value: jlong,
) -> jni::errors::Result<()> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    unsafe {
        do_tx_op(
            env,
            tx.into(),
            InsertOp {
                obj,
                index: idx,
                value: am::ScalarValue::counter(value),
            },
        )
    }
}

fn insert_date_in_list<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    idx: jlong,
    date: JObject<'local>,
) -> jni::errors::Result<()> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    let date_millis = env
        .call_method(date, jni_str!("getTime"), jni_sig!("()J"), &[])?
        .j()?;
    unsafe {
        do_tx_op(
            env,
            tx.into(),
            InsertOp {
                obj,
                index: idx,
                value: am::ScalarValue::Timestamp(date_millis),
            },
        )
    }
}

fn insert_bool_in_list<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    idx: jlong,
    value: jboolean,
) -> jni::errors::Result<()> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    unsafe {
        do_tx_op(
            env,
            tx.into(),
            InsertOp {
                obj,
                index: idx,
                value: am::ScalarValue::Boolean(value),
            },
        )
    }
}

fn insert_object_in_list<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    idx: jlong,
    value: JObject<'local>,
) -> jni::errors::Result<bindings::ObjectId<'local>> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    let result = unsafe {
        let obj_type = JavaObjType::from_java_enum(env, value)?;
        do_tx_op(
            env,
            tx.into(),
            InsertOp {
                obj,
                index: idx,
                value: am::ObjType::from(obj_type),
            },
        )?
    };
    result.into_object_id(env)
}
