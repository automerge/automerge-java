use jni::{
    objects::{JClass, JObjectArray},
    NativeMethod,
};

use super::SomeReadPointer;

use crate::bindings;

const _METHODS: &[NativeMethod] = &[
    ams_native! { static extern fn get_map_entries_in_tx(tx: bindings::TransactionPointer, obj: bindings::ObjectId) -> bindings::Optional },
    ams_native! { static extern fn get_map_entries_in_doc(doc: bindings::DocPointer, obj: bindings::ObjectId) -> bindings::Optional },
    ams_native! { static extern fn get_map_entries_at_in_tx(tx: bindings::TransactionPointer, obj: bindings::ObjectId, heads: bindings::ChangeHash[]) -> bindings::Optional },
    ams_native! { static extern fn get_map_entries_at_in_doc(doc: bindings::DocPointer, obj: bindings::ObjectId, heads: bindings::ChangeHash[]) -> bindings::Optional },
];

fn get_map_entries_in_tx<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
) -> jni::errors::Result<bindings::Optional<'local>> {
    unsafe { SomeReadPointer::tx(tx.into()).map_entries(env, obj.into(), None) }
}

fn get_map_entries_in_doc<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
    obj: bindings::ObjectId<'local>,
) -> jni::errors::Result<bindings::Optional<'local>> {
    unsafe { SomeReadPointer::doc(doc.into()).map_entries(env, obj.into(), None) }
}

fn get_map_entries_at_in_tx<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    heads: JObjectArray<'local, bindings::ChangeHash<'local>>,
) -> jni::errors::Result<bindings::Optional<'local>> {
    unsafe { SomeReadPointer::tx(tx.into()).map_entries(env, obj.into(), Some(heads)) }
}

fn get_map_entries_at_in_doc<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
    obj: bindings::ObjectId<'local>,
    heads: JObjectArray<'local, bindings::ChangeHash<'local>>,
) -> jni::errors::Result<bindings::Optional<'local>> {
    unsafe { SomeReadPointer::doc(doc.into()).map_entries(env, obj.into(), Some(heads)) }
}
