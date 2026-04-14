use jni::{
    objects::{JClass, JObjectArray},
    NativeMethod,
};

use super::SomeReadPointer;

use crate::bindings;

const _METHODS: &[NativeMethod] = &[
    ams_native! { static extern fn get_heads_in_tx(tx: bindings::TransactionPointer) -> bindings::ChangeHash[] },
    ams_native! { static extern fn get_heads_in_doc(doc_pointer: bindings::DocPointer) -> bindings::ChangeHash[] },
];

fn get_heads_in_tx<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    tx: bindings::TransactionPointer<'local>,
) -> jni::errors::Result<JObjectArray<'local, bindings::ChangeHash<'local>>> {
    unsafe { SomeReadPointer::tx(tx.into()).heads(env) }
}

fn get_heads_in_doc<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    doc_pointer: bindings::DocPointer<'local>,
) -> jni::errors::Result<JObjectArray<'local, bindings::ChangeHash<'local>>> {
    unsafe { SomeReadPointer::doc(doc_pointer.into()).heads(env) }
}
