use jni::{objects::JClass, NativeMethod};

use super::SomeReadPointer;

use crate::bindings;

const _METHODS: &[NativeMethod] = &[
    ams_native! { static extern fn get_object_type_in_doc(doc: bindings::DocPointer, obj: bindings::ObjectId) -> bindings::Optional },
    ams_native! { static extern fn get_object_type_in_tx(tx: bindings::TransactionPointer, obj: bindings::ObjectId) -> bindings::Optional },
];

fn get_object_type_in_doc<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc: bindings::DocPointer<'local>,
    obj: bindings::ObjectId<'local>,
) -> jni::errors::Result<bindings::Optional<'local>> {
    unsafe { SomeReadPointer::doc(doc.into()).get_object_type(env, obj.into()) }
}

fn get_object_type_in_tx<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
    obj: bindings::ObjectId<'local>,
) -> jni::errors::Result<bindings::Optional<'local>> {
    unsafe { SomeReadPointer::tx(tx.into()).get_object_type(env, obj.into()) }
}
