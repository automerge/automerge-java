use jni::{
    objects::{JObject, JPrimitiveArray, JValue},
    sys::jbyte,
    JNIEnv,
};
use samod_core::DocumentId;

pub(crate) const DOCUMENT_ID_CLASS: &str = am_classname!("DocumentId");

pub(crate) fn document_id_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    doc_id: &DocumentId,
) -> Result<JObject<'local>, jni::errors::Error> {
    let uuid_bytes = doc_id.as_bytes(); // 16-byte UUID representation

    // Create Java byte array from UUID bytes
    let java_byte_array = env.byte_array_from_slice(uuid_bytes)?;

    // Create DocumentId object using constructor
    let args = [JValue::from(&java_byte_array)];
    let obj = env.new_object(DOCUMENT_ID_CLASS, "([B)V", &args)?;

    Ok(obj)
}

pub(crate) fn java_object_to_document_id(
    env: &mut JNIEnv,
    obj: JObject,
) -> Result<DocumentId, jni::errors::Error> {
    // Get the bytes field
    let bytes_field = env.get_field(&obj, "bytes", "[B")?;
    let java_byte_array = bytes_field.l()?;
    let jbytearray = JPrimitiveArray::<jbyte>::from(java_byte_array);
    let bytes = env.convert_byte_array(&jbytearray)?;

    // Use DocumentId::try_from(Vec<u8>) which internally calls Uuid::from_slice()
    DocumentId::try_from(bytes).map_err(|_e| jni::errors::Error::JavaException)
}
