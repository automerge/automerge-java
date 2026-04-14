use jni::{
    objects::{JClass, JString},
    sys::jlong,
    NativeMethod,
};

use super::SomeReadPointer;

use crate::bindings;

const _METHODS: &[NativeMethod] = &[
    ams_native! { static extern fn get_in_map_in_doc(doc: bindings::DocPointer, obj: bindings::ObjectId, key: JString) -> bindings::Optional },
    ams_native! { static extern fn get_in_map_in_tx(tx: bindings::TransactionPointer, obj: bindings::ObjectId, key: JString) -> bindings::Optional },
    ams_native! { static extern fn get_in_list_in_doc(doc: bindings::DocPointer, obj: bindings::ObjectId, idx: jlong) -> bindings::Optional },
    ams_native! { static extern fn get_in_list_in_tx(tx: bindings::TransactionPointer, obj: bindings::ObjectId, idx: jlong) -> bindings::Optional },
];

fn get_in_map_in_doc<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
    obj: bindings::ObjectId<'local>,
    key: JString<'local>,
) -> jni::errors::Result<bindings::Optional<'local>> {
    unsafe { SomeReadPointer::doc(doc.into()).get(env, obj.into(), key) }
}

fn get_in_map_in_tx<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    key: JString<'local>,
) -> jni::errors::Result<bindings::Optional<'local>> {
    unsafe { SomeReadPointer::tx(tx.into()).get(env, obj.into(), key) }
}

fn get_in_list_in_doc<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
    obj: bindings::ObjectId<'local>,
    idx: jlong,
) -> jni::errors::Result<bindings::Optional<'local>> {
    unsafe { SomeReadPointer::doc(doc.into()).get(env, obj.into(), idx) }
}

fn get_in_list_in_tx<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
    idx: jlong,
) -> jni::errors::Result<bindings::Optional<'local>> {
    unsafe { SomeReadPointer::tx(tx.into()).get(env, obj.into(), idx) }
}
