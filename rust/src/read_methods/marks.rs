use jni::{objects::JClass, NativeMethod};

use crate::bindings;

use super::SomeReadPointer;

const _METHODS: &[NativeMethod] = &[
    ams_native! { static extern fn get_marks_in_doc(doc: bindings::DocPointer, obj: bindings::ObjectId, heads: java.util.Optional) -> bindings::ArrayList },
    ams_native! { static extern fn get_marks_in_tx(tx: bindings::TransactionPointer, obj: bindings::ObjectId, heads: java.util.Optional) -> bindings::ArrayList },
    ams_native! { static extern fn get_marks_at_index_in_doc(doc: bindings::DocPointer, obj: bindings::ObjectId, index: jlong, heads: java.util.Optional) -> java.util.HashMap },
    ams_native! { static extern fn get_marks_at_index_in_tx(tx: bindings::TransactionPointer, obj: bindings::ObjectId, index: jlong, heads: java.util.Optional) -> java.util.HashMap },
];

fn get_marks_in_doc<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
    obj: bindings::ObjectId<'local>,
    heads: bindings::Optional<'local>,
) -> jni::errors::Result<bindings::ArrayList<'local>> {
    unsafe { SomeReadPointer::doc(doc.into()).marks(env, obj.into(), heads) }
}

fn get_marks_in_tx<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    heads: bindings::Optional<'local>,
) -> jni::errors::Result<bindings::ArrayList<'local>> {
    unsafe { SomeReadPointer::tx(tx.into()).marks(env, obj.into(), heads) }
}

fn get_marks_at_index_in_doc<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
    obj: bindings::ObjectId<'local>,
    index: jni::sys::jlong,
    heads: bindings::Optional<'local>,
) -> jni::errors::Result<bindings::HashMap<'local>> {
    unsafe {
        SomeReadPointer::doc(doc.into()).marks_at_index(
            env,
            obj.into(),
            index as jni::sys::jint,
            heads,
        )
    }
}

fn get_marks_at_index_in_tx<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    index: jni::sys::jlong,
    heads: bindings::Optional<'local>,
) -> jni::errors::Result<bindings::HashMap<'local>> {
    unsafe {
        SomeReadPointer::tx(tx.into()).marks_at_index(
            env,
            obj.into(),
            index as jni::sys::jint,
            heads,
        )
    }
}
