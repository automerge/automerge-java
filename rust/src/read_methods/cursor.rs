use jni::{objects::JClass, sys::jlong, NativeMethod};

use super::SomeReadPointer;

use crate::bindings;

const _METHODS: &[NativeMethod] = &[
    ams_native! { static extern fn make_cursor_in_doc(doc: bindings::DocPointer, obj: bindings::ObjectId, index: jlong, heads: java.util.Optional) -> bindings::Cursor },
    ams_native! { static extern fn make_cursor_in_tx(tx: bindings::TransactionPointer, obj: bindings::ObjectId, index: jlong, heads: java.util.Optional) -> bindings::Cursor },
    ams_native! { static extern fn lookup_cursor_index_in_doc(doc: bindings::DocPointer, obj: bindings::ObjectId, cursor: bindings::Cursor, heads: java.util.Optional) -> jlong },
    ams_native! { static extern fn lookup_cursor_index_in_tx(tx: bindings::TransactionPointer, obj: bindings::ObjectId, cursor: bindings::Cursor, heads: java.util.Optional) -> jlong },
];

fn make_cursor_in_doc<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
    obj: bindings::ObjectId<'local>,
    index: jlong,
    heads: bindings::Optional<'local>,
) -> jni::errors::Result<bindings::Cursor<'local>> {
    unsafe { SomeReadPointer::doc(doc.into()).make_cursor(env, obj.into(), index, heads) }
}

fn make_cursor_in_tx<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    index: jlong,
    heads: bindings::Optional<'local>,
) -> jni::errors::Result<bindings::Cursor<'local>> {
    unsafe { SomeReadPointer::tx(tx.into()).make_cursor(env, obj.into(), index, heads) }
}

fn lookup_cursor_index_in_doc<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
    obj: bindings::ObjectId<'local>,
    cursor: bindings::Cursor<'local>,
    heads: bindings::Optional<'local>,
) -> jni::errors::Result<jlong> {
    unsafe {
        SomeReadPointer::doc(doc.into()).lookup_cursor_index(env, obj.into(), cursor.into(), heads)
    }
}

fn lookup_cursor_index_in_tx<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    cursor: bindings::Cursor<'local>,
    heads: bindings::Optional<'local>,
) -> jni::errors::Result<jlong> {
    unsafe {
        SomeReadPointer::tx(tx.into()).lookup_cursor_index(env, obj.into(), cursor.into(), heads)
    }
}
