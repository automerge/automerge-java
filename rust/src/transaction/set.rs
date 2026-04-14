use automerge as am;
use automerge::transaction::Transactable;
use jni::objects::{JByteArray, JClass};
use jni::sys::jboolean;
use jni::{jni_sig, jni_str};
use jni::{
    objects::{JObject, JString},
    sys::jlong,
    NativeMethod,
};

use crate::interop::{read_usize, unwrap_or_throw_amg_exc};
use crate::obj_id::JavaObjId;
use crate::obj_type::JavaObjType;
use crate::prop::JProp;

use super::{do_tx_op, TransactionOp};

struct SetOp<'a, V: Into<automerge::ScalarValue>> {
    obj: JavaObjId,
    prop: JProp<'a>,
    value: V,
}

impl<'a, V: Into<automerge::ScalarValue>> TransactionOp for SetOp<'a, V> {
    type Output<'local> = ();

    unsafe fn execute<'local, T: Transactable>(
        self,
        env: &jni::Env<'local>,
        tx: &mut T,
    ) -> Result<Self::Output<'local>, jni::errors::Error> {
        let key = self.prop.try_into_prop(env)?;

        unwrap_or_throw_amg_exc(env, tx.put(self.obj, key, self.value))
    }
}

use crate::bindings;

const _METHODS: &[NativeMethod] = &[
    ams_native! { static extern fn set_double_in_map(tx: bindings::TransactionPointer, obj: bindings::ObjectId, key: JString, value: jdouble) },
    ams_native! { static extern fn set_bytes_in_map(tx: bindings::TransactionPointer, obj: bindings::ObjectId, key: JString, value: jbyte[]) },
    ams_native! { static extern fn set_string_in_map(tx: bindings::TransactionPointer, obj: bindings::ObjectId, key: JString, value: JString) },
    ams_native! { static extern fn set_int_in_map(tx: bindings::TransactionPointer, obj: bindings::ObjectId, key: JString, value: jlong) },
    ams_native! { static extern fn set_uint_in_map(tx: bindings::TransactionPointer, obj: bindings::ObjectId, key: JString, value: jlong) },
    ams_native! { static extern fn set_bool_in_map(tx: bindings::TransactionPointer, obj: bindings::ObjectId, key: JString, value: jboolean) },
    ams_native! { static extern fn set_null_in_map(tx: bindings::TransactionPointer, obj: bindings::ObjectId, key: JString) },
    ams_native! { static extern fn set_counter_in_map(tx: bindings::TransactionPointer, obj: bindings::ObjectId, key: JString, value: jlong) },
    ams_native! { static extern fn set_date_in_map(tx: bindings::TransactionPointer, obj: bindings::ObjectId, key: JString, date: java.util.Date) },
    ams_native! { static extern fn set_double_in_list(tx: bindings::TransactionPointer, obj: bindings::ObjectId, idx: jlong, value: jdouble) },
    ams_native! { static extern fn set_int_in_list(tx: bindings::TransactionPointer, obj: bindings::ObjectId, idx: jlong, value: jlong) },
    ams_native! { static extern fn set_uint_in_list(tx: bindings::TransactionPointer, obj: bindings::ObjectId, idx: jlong, value: jlong) },
    ams_native! { static extern fn set_string_in_list(tx: bindings::TransactionPointer, obj: bindings::ObjectId, idx: jlong, value: JString) },
    ams_native! { static extern fn set_bytes_in_list(tx: bindings::TransactionPointer, obj: bindings::ObjectId, idx: jlong, value: jbyte[]) },
    ams_native! { static extern fn set_bool_in_list(tx: bindings::TransactionPointer, obj: bindings::ObjectId, idx: jlong, value: jboolean) },
    ams_native! { static extern fn set_date_in_list(tx: bindings::TransactionPointer, obj: bindings::ObjectId, idx: jlong, date: java.util.Date) },
    ams_native! { static extern fn set_counter_in_list(tx: bindings::TransactionPointer, obj: bindings::ObjectId, idx: jlong, value: jlong) },
    ams_native! { static extern fn set_null_in_list(tx: bindings::TransactionPointer, obj: bindings::ObjectId, idx: jlong) },
    ams_native! { static extern fn set_object_in_map(tx: bindings::TransactionPointer, obj: bindings::ObjectId, key: JString, value: org.automerge.ObjectType) -> bindings::ObjectId },
    ams_native! { static extern fn set_object_in_list(tx: bindings::TransactionPointer, obj: bindings::ObjectId, idx: jlong, value: org.automerge.ObjectType) -> bindings::ObjectId },
];

// The Java signature for setIntInMap is `long value` but was historically
// called `setIntInMap` — keeping the original jint parameter type for the Java
// int maps to a Java long (jlong). Need to match what Java declares.

fn set_double_in_map<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    key: JString<'local>,
    value: jni::sys::jdouble,
) -> jni::errors::Result<()> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    unsafe {
        do_tx_op(
            env,
            tx.into(),
            SetOp {
                obj,
                prop: key.into(),
                value,
            },
        )
    }
}

fn set_bytes_in_map<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    key: JString<'local>,
    value: JByteArray<'local>,
) -> jni::errors::Result<()> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    let bytes = env.convert_byte_array(value)?;
    unsafe {
        do_tx_op(
            env,
            tx.into(),
            SetOp {
                obj,
                prop: key.into(),
                value: bytes.as_slice().to_vec(),
            },
        )
    }
}

fn set_string_in_map<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    key: JString<'local>,
    value: JString<'local>,
) -> jni::errors::Result<()> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    unsafe {
        do_tx_op(
            env,
            tx.into(),
            SetOp {
                obj,
                prop: key.into(),
                value: value.to_string(),
            },
        )
    }
}

fn set_int_in_map<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    key: JString<'local>,
    value: jlong,
) -> jni::errors::Result<()> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    unsafe {
        do_tx_op(
            env,
            tx.into(),
            SetOp {
                obj,
                prop: key.into(),
                value,
            },
        )
    }
}

fn set_uint_in_map<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    key: JString<'local>,
    value: jlong,
) -> jni::errors::Result<()> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    unsafe {
        do_tx_op(
            env,
            tx.into(),
            SetOp {
                obj,
                prop: key.into(),
                value: am::ScalarValue::Uint(value as u64),
            },
        )
    }
}

fn set_bool_in_map<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    key: JString<'local>,
    value: jboolean,
) -> jni::errors::Result<()> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    unsafe {
        do_tx_op(
            env,
            tx.into(),
            SetOp {
                obj,
                prop: key.into(),
                value: am::ScalarValue::Boolean(value),
            },
        )
    }
}

fn set_null_in_map<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    key: JString<'local>,
) -> jni::errors::Result<()> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    unsafe {
        do_tx_op(
            env,
            tx.into(),
            SetOp {
                obj,
                prop: key.into(),
                value: am::ScalarValue::Null,
            },
        )
    }
}

fn set_counter_in_map<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    key: JString<'local>,
    value: jlong,
) -> jni::errors::Result<()> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    unsafe {
        do_tx_op(
            env,
            tx.into(),
            SetOp {
                obj,
                prop: key.into(),
                value: am::ScalarValue::counter(value),
            },
        )
    }
}

fn set_date_in_map<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    key: JString<'local>,
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
            SetOp {
                obj,
                prop: key.into(),
                value: am::ScalarValue::Timestamp(date_millis),
            },
        )
    }
}

fn set_double_in_list<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    idx: jlong,
    value: jni::sys::jdouble,
) -> jni::errors::Result<()> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    unsafe {
        do_tx_op(
            env,
            tx.into(),
            SetOp {
                obj,
                prop: idx.into(),
                value,
            },
        )
    }
}

fn set_int_in_list<'local>(
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
            SetOp {
                obj,
                prop: idx.into(),
                value,
            },
        )
    }
}

fn set_uint_in_list<'local>(
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
            SetOp {
                obj,
                prop: idx.into(),
                value: am::ScalarValue::Uint(value as u64),
            },
        )
    }
}

fn set_string_in_list<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    idx: jlong,
    value: JString<'local>,
) -> jni::errors::Result<()> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    unsafe {
        do_tx_op(
            env,
            tx.into(),
            SetOp {
                obj,
                prop: idx.into(),
                value: value.to_string(),
            },
        )
    }
}

fn set_bytes_in_list<'local>(
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
            SetOp {
                obj,
                prop: idx.into(),
                value: bytes,
            },
        )
    }
}

fn set_bool_in_list<'local>(
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
            SetOp {
                obj,
                prop: idx.into(),
                value: am::ScalarValue::Boolean(value),
            },
        )
    }
}

fn set_date_in_list<'local>(
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
            SetOp {
                obj,
                prop: idx.into(),
                value: am::ScalarValue::Timestamp(date_millis),
            },
        )
    }
}

fn set_counter_in_list<'local>(
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
            SetOp {
                obj,
                prop: idx.into(),
                value: am::ScalarValue::counter(value),
            },
        )
    }
}

fn set_null_in_list<'local>(
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
            SetOp {
                obj,
                prop: idx.into(),
                value: am::ScalarValue::Null,
            },
        )
    }
}

struct SetObjOp {
    obj: JavaObjId,
    key: automerge::Prop,
    value: am::ObjType,
}

impl TransactionOp for SetObjOp {
    type Output<'local> = JavaObjId;

    unsafe fn execute<'local, T: Transactable>(
        self,
        env: &jni::Env<'local>,
        tx: &mut T,
    ) -> Result<Self::Output<'local>, jni::errors::Error> {
        Ok(unwrap_or_throw_amg_exc(env, tx.put_object(self.obj, self.key, self.value))?.into())
    }
}

fn set_object_in_map<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    key: JString<'local>,
    value: JObject<'local>,
) -> jni::errors::Result<bindings::ObjectId<'local>> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    let key = key.to_string();
    let obj_id = unsafe {
        let obj_type = JavaObjType::from_java_enum(env, value)?;
        do_tx_op(
            env,
            tx.into(),
            SetObjOp {
                obj,
                key: key.into(),
                value: obj_type.into(),
            },
        )?
    };
    obj_id.into_object_id(env)
}

fn set_object_in_list<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    idx: jlong,
    value: JObject<'local>,
) -> jni::errors::Result<bindings::ObjectId<'local>> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    let idx = read_usize(env, idx)?;
    let result = unsafe {
        let obj_type = JavaObjType::from_java_enum(env, value)?;
        do_tx_op(
            env,
            tx.into(),
            SetObjOp {
                obj,
                key: idx.into(),
                value: obj_type.into(),
            },
        )?
    };
    result.into_object_id(env)
}
