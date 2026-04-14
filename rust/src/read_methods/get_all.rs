use jni::{
    objects::{JClass, JObjectArray, JString},
    sys::jlong,
    NativeMethod,
};

use super::SomeReadPointer;

use crate::bindings;

const _METHODS: &[NativeMethod] = &[
    ams_native! { static extern fn get_all_in_map_in_doc(doc: bindings::DocPointer, obj: bindings::ObjectId, key: JString) -> bindings::Optional },
    ams_native! { static extern fn get_all_in_map_in_tx(tx: bindings::TransactionPointer, obj: bindings::ObjectId, key: JString) -> bindings::Optional },
    ams_native! { static extern fn get_all_in_list_in_doc(doc: bindings::DocPointer, obj: bindings::ObjectId, idx: jlong) -> bindings::Optional },
    ams_native! { static extern fn get_all_in_list_in_tx(tx: bindings::TransactionPointer, obj: bindings::ObjectId, idx: jlong) -> bindings::Optional },
    ams_native! { static extern fn get_all_at_in_map_in_doc(doc: bindings::DocPointer, obj: bindings::ObjectId, key: JString, heads: bindings::ChangeHash[]) -> bindings::Optional },
    ams_native! { static extern fn get_all_at_in_map_in_tx(tx: bindings::TransactionPointer, obj: bindings::ObjectId, key: JString, heads: bindings::ChangeHash[]) -> bindings::Optional },
    ams_native! { static extern fn get_all_at_in_list_in_doc(doc: bindings::DocPointer, obj: bindings::ObjectId, idx: jlong, heads: bindings::ChangeHash[]) -> bindings::Optional },
    ams_native! { static extern fn get_all_at_in_list_in_tx(tx: bindings::TransactionPointer, obj: bindings::ObjectId, idx: jlong, heads: bindings::ChangeHash[]) -> bindings::Optional },
];

fn get_all_in_map_in_doc<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
    obj: bindings::ObjectId<'local>,
    key: JString<'local>,
) -> jni::errors::Result<bindings::Optional<'local>> {
    unsafe { SomeReadPointer::doc(doc.into()).get_all(env, obj.into(), key, None) }
}

fn get_all_in_map_in_tx<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    key: JString<'local>,
) -> jni::errors::Result<bindings::Optional<'local>> {
    unsafe { SomeReadPointer::tx(tx.into()).get_all(env, obj.into(), key, None) }
}

fn get_all_in_list_in_doc<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
    obj: bindings::ObjectId<'local>,
    idx: jlong,
) -> jni::errors::Result<bindings::Optional<'local>> {
    unsafe { SomeReadPointer::doc(doc.into()).get_all(env, obj.into(), idx, None) }
}

fn get_all_in_list_in_tx<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    idx: jlong,
) -> jni::errors::Result<bindings::Optional<'local>> {
    unsafe { SomeReadPointer::tx(tx.into()).get_all(env, obj.into(), idx, None) }
}

fn get_all_at_in_map_in_doc<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
    obj: bindings::ObjectId<'local>,
    key: JString<'local>,
    heads: JObjectArray<'local, bindings::ChangeHash<'local>>,
) -> jni::errors::Result<bindings::Optional<'local>> {
    unsafe { SomeReadPointer::doc(doc.into()).get_all(env, obj.into(), key, Some(heads)) }
}

fn get_all_at_in_map_in_tx<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    key: JString<'local>,
    heads: JObjectArray<'local, bindings::ChangeHash<'local>>,
) -> jni::errors::Result<bindings::Optional<'local>> {
    unsafe { SomeReadPointer::tx(tx.into()).get_all(env, obj.into(), key, Some(heads)) }
}

fn get_all_at_in_list_in_doc<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
    obj: bindings::ObjectId<'local>,
    idx: jlong,
    heads: JObjectArray<'local, bindings::ChangeHash<'local>>,
) -> jni::errors::Result<bindings::Optional<'local>> {
    unsafe { SomeReadPointer::doc(doc.into()).get_all(env, obj.into(), idx, Some(heads)) }
}

fn get_all_at_in_list_in_tx<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    idx: jlong,
    heads: JObjectArray<'local, bindings::ChangeHash<'local>>,
) -> jni::errors::Result<bindings::Optional<'local>> {
    unsafe { SomeReadPointer::tx(tx.into()).get_all(env, obj.into(), idx, Some(heads)) }
}
