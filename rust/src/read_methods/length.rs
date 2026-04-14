use jni::{
    objects::{JClass, JObjectArray},
    sys::jlong,
    NativeMethod,
};

use super::SomeReadPointer;

use crate::bindings;

const _METHODS: &[NativeMethod] = &[
    ams_native! { static extern fn get_list_length_in_tx(tx: bindings::TransactionPointer, obj: bindings::ObjectId) -> jlong },
    ams_native! { static extern fn get_list_length_in_doc(doc: bindings::DocPointer, obj: bindings::ObjectId) -> jlong },
    ams_native! { static extern fn get_list_length_at_in_tx(tx: bindings::TransactionPointer, obj: bindings::ObjectId, heads: bindings::ChangeHash[]) -> jlong },
    ams_native! { static extern fn get_list_length_at_in_doc(doc: bindings::DocPointer, obj: bindings::ObjectId, heads: bindings::ChangeHash[]) -> jlong },
];

fn get_list_length_in_tx<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
) -> jni::errors::Result<jlong> {
    unsafe { SomeReadPointer::tx(tx.into()).length(env, obj.into(), None) }
}

fn get_list_length_in_doc<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
    obj: bindings::ObjectId<'local>,
) -> jni::errors::Result<jlong> {
    unsafe { SomeReadPointer::doc(doc.into()).length(env, obj.into(), None) }
}

fn get_list_length_at_in_tx<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    heads: JObjectArray<'local, bindings::ChangeHash<'local>>,
) -> jni::errors::Result<jlong> {
    unsafe { SomeReadPointer::tx(tx.into()).length(env, obj.into(), Some(heads)) }
}

fn get_list_length_at_in_doc<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
    obj: bindings::ObjectId<'local>,
    heads: JObjectArray<'local, bindings::ChangeHash<'local>>,
) -> jni::errors::Result<jlong> {
    unsafe { SomeReadPointer::doc(doc.into()).length(env, obj.into(), Some(heads)) }
}
