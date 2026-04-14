use am::transaction::Transactable;
use automerge::{self as am, marks::ExpandMark, ScalarValue};
use jni::{
    jni_sig, jni_str,
    objects::{JByteArray, JClass, JObject, JString},
    sys::{jboolean, jdouble, jlong},
    NativeMethod,
};

use crate::{
    expand_mark,
    interop::{read_u64, unwrap_or_throw_amg_exc},
    obj_id::JavaObjId,
};

use super::{do_tx_op, TransactionOp};

struct MarkOp {
    obj: JavaObjId,
    start: usize,
    end: usize,
    name: String,
    value: am::ScalarValue,
    expand: ExpandMark,
}

impl TransactionOp for MarkOp {
    type Output<'local> = ();

    unsafe fn execute<'local, T: Transactable>(
        self,
        env: &jni::Env<'local>,
        tx: &mut T,
    ) -> Result<Self::Output<'local>, jni::errors::Error> {
        let mark = am::marks::Mark::new(self.name, self.value, self.start, self.end);
        unwrap_or_throw_amg_exc(env, tx.mark(self.obj, mark, self.expand))
    }
}

#[allow(clippy::too_many_arguments)]
unsafe fn do_mark<'local, V: Into<ScalarValue>>(
    env: &mut jni::Env<'local>,
    tx: JObject<'local>,
    obj: JObject<'local>,
    name: JString<'local>,
    start: jlong,
    end: jlong,
    value: V,
    expand: bindings::ExpandMark<'local>,
) -> Result<(), jni::errors::Error> {
    let obj = JavaObjId::from_jobject(env, obj)?;
    let expand = expand_mark::from_java(env, expand)?;
    do_tx_op(
        env,
        tx,
        MarkOp {
            obj,
            start: start as usize,
            end: end as usize,
            name: name.to_string(),
            value: value.into(),
            expand,
        },
    )
}

use crate::bindings;

const _METHODS: &[NativeMethod] = &[
    ams_native! { static extern fn mark_uint(
        tx: bindings::TransactionPointer,
        obj: bindings::ObjectId,
        name: JString,
        start: jlong,
        end: jlong,
        value: jlong,
        expand: bindings::ExpandMark
    ) },
    ams_native! { static extern fn mark_int(
        tx: bindings::TransactionPointer,
        obj: bindings::ObjectId,
        name: JString,
        start: jlong,
        end: jlong,
        value: jlong,
        expand: bindings::ExpandMark
    ) },
    ams_native! { static extern fn mark_double(tx: bindings::TransactionPointer,
        obj: bindings::ObjectId,
        name: JString,
        start: jlong,
        end: jlong,
        value: jdouble,
        expand: bindings::ExpandMark
    ) },
    ams_native! { static extern fn mark_bytes(tx: bindings::TransactionPointer,
        obj: bindings::ObjectId,
        name: JString,
        start: jlong,
        end: jlong,
        value: jbyte[],
        expand: bindings::ExpandMark
    ) },
    ams_native! { static extern fn mark_string(tx: bindings::TransactionPointer,
        obj: bindings::ObjectId,
        name: JString,
        start: jlong,
        end: jlong,
        value: JString,
        expand: bindings::ExpandMark
    ) },
    ams_native! { static extern fn mark_counter(tx: bindings::TransactionPointer,
        obj: bindings::ObjectId,
        name: JString,
        start: jlong,
        end: jlong,
        value: jlong,
        expand: bindings::ExpandMark
    ) },
    ams_native! { static extern fn mark_date(tx: bindings::TransactionPointer,
        obj: bindings::ObjectId,
        name: JString,
        start: jlong,
        end: jlong,
        date: java.util.Date,
        expand: bindings::ExpandMark
    ) },
    ams_native! { static extern fn mark_bool(tx: bindings::TransactionPointer,
        obj: bindings::ObjectId,
        name: JString,
        start: jlong,
        end: jlong,
        value: jboolean,
        expand: bindings::ExpandMark
    ) },
    ams_native! { static extern fn mark_null(tx: bindings::TransactionPointer,
        obj: bindings::ObjectId,
        name: JString,
        start: jlong,
        end: jlong,
        expand: bindings::ExpandMark
    ) },
    ams_native! { static extern fn un_mark(tx: bindings::TransactionPointer,
        obj: bindings::ObjectId,
        name: JString,
        start: jlong,
        end: jlong,
        expand: bindings::ExpandMark
    ) },
];

#[allow(clippy::too_many_arguments)]
fn mark_uint<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    name: JString<'local>,
    start: jlong,
    end: jlong,
    value: jlong,
    expand: bindings::ExpandMark<'local>,
) -> jni::errors::Result<()> {
    let value = read_u64(env, value)?;
    unsafe { do_mark(env, tx.into(), obj.into(), name, start, end, value, expand) }
}

#[allow(clippy::too_many_arguments)]
fn mark_int<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    name: JString<'local>,
    start: jlong,
    end: jlong,
    value: jlong,
    expand: bindings::ExpandMark<'local>,
) -> jni::errors::Result<()> {
    unsafe { do_mark(env, tx.into(), obj.into(), name, start, end, value, expand) }
}

#[allow(clippy::too_many_arguments)]
fn mark_double<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    name: JString<'local>,
    start: jlong,
    end: jlong,
    value: jdouble,
    expand: bindings::ExpandMark<'local>,
) -> jni::errors::Result<()> {
    unsafe { do_mark(env, tx.into(), obj.into(), name, start, end, value, expand) }
}

#[allow(clippy::too_many_arguments)]
fn mark_bytes<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    name: JString<'local>,
    start: jlong,
    end: jlong,
    value: JByteArray<'local>,
    expand: bindings::ExpandMark<'local>,
) -> jni::errors::Result<()> {
    let value = env.convert_byte_array(value)?;
    unsafe { do_mark(env, tx.into(), obj.into(), name, start, end, value, expand) }
}

#[allow(clippy::too_many_arguments)]
fn mark_string<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    name: JString<'local>,
    start: jlong,
    end: jlong,
    value: JString<'local>,
    expand: bindings::ExpandMark<'local>,
) -> jni::errors::Result<()> {
    let value = value.to_string();
    unsafe { do_mark(env, tx.into(), obj.into(), name, start, end, value, expand) }
}

#[allow(clippy::too_many_arguments)]
fn mark_counter<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    name: JString<'local>,
    start: jlong,
    end: jlong,
    value: jlong,
    expand: bindings::ExpandMark<'local>,
) -> jni::errors::Result<()> {
    let value = ScalarValue::Counter(value.into());
    unsafe { do_mark(env, tx.into(), obj.into(), name, start, end, value, expand) }
}

#[allow(clippy::too_many_arguments)]
fn mark_date<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    name: JString<'local>,
    start: jlong,
    end: jlong,
    date: JObject<'local>,
    expand: bindings::ExpandMark<'local>,
) -> jni::errors::Result<()> {
    let date_millis = env
        .call_method(date, jni_str!("getTime"), jni_sig!("()J"), &[])?
        .j()?;
    unsafe {
        do_mark(
            env,
            tx.into(),
            obj.into(),
            name,
            start,
            end,
            ScalarValue::Timestamp(date_millis),
            expand,
        )
    }
}

#[allow(clippy::too_many_arguments)]
fn mark_bool<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    name: JString<'local>,
    start: jlong,
    end: jlong,
    value: jboolean,
    expand: bindings::ExpandMark<'local>,
) -> jni::errors::Result<()> {
    unsafe {
        do_mark(
            env,
            tx.into(),
            obj.into(),
            name,
            start,
            end,
            am::ScalarValue::Boolean(value),
            expand,
        )
    }
}

#[expect(clippy::too_many_arguments)]
fn mark_null<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    name: JString<'local>,
    start: jlong,
    end: jlong,
    expand: bindings::ExpandMark<'local>,
) -> jni::errors::Result<()> {
    unsafe {
        do_mark(
            env,
            tx.into(),
            obj.into(),
            name,
            start,
            end,
            am::ScalarValue::Null,
            expand,
        )
    }
}

struct Unmark {
    obj: JavaObjId,
    start: usize,
    end: usize,
    name: String,
    expand: ExpandMark,
}

impl TransactionOp for Unmark {
    type Output<'local> = ();

    unsafe fn execute<'local, T: Transactable>(
        self,
        env: &jni::Env<'local>,
        tx: &mut T,
    ) -> Result<Self::Output<'local>, jni::errors::Error> {
        unwrap_or_throw_amg_exc(
            env,
            tx.unmark(self.obj, &self.name, self.start, self.end, self.expand),
        )
    }
}

#[expect(clippy::too_many_arguments)]
fn un_mark<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    name: JString<'local>,
    start: jlong,
    end: jlong,
    expand: bindings::ExpandMark<'local>,
) -> jni::errors::Result<()> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    let expand = expand_mark::from_java(env, expand)?;
    let name = name.to_string();
    unsafe {
        do_tx_op(
            env,
            tx.into(),
            Unmark {
                obj,
                start: start as usize,
                end: end as usize,
                name,
                expand,
            },
        )
    }
}
