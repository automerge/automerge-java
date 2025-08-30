use jni::objects::{JList, JObject, JValue};
use jni::JNIEnv;
use samod_core::DocumentChanged;

pub(crate) fn document_changed_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    document_changed: &DocumentChanged,
) -> Result<JObject<'local>, jni::errors::Error> {
    let arraylist_class = env.find_class("java/util/ArrayList")?;
    let arraylist_obj = env.new_object(arraylist_class, "()V", &[])?;
    let arraylist = JList::from_env(env, &arraylist_obj)?;

    for change_hash in &document_changed.new_heads {
        let change_hash_obj = change_hash_to_java_object(env, change_hash)?;
        arraylist.add(env, &change_hash_obj)?;
    }

    let list_obj = arraylist_obj;

    let class = env.find_class(am_classname!("DocumentChanged"))?;
    let obj = env.new_object(class, "(Ljava/util/List;)V", &[JValue::Object(&list_obj)])?;
    Ok(obj)
}

fn change_hash_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    change_hash: &automerge::ChangeHash,
) -> Result<JObject<'local>, jni::errors::Error> {
    let bytes = change_hash.as_ref();
    let byte_array = env.byte_array_from_slice(bytes)?;
    let class = env.find_class(am_classname!("ChangeHash"))?;
    let obj = env.new_object(class, "([B)V", &[JValue::Object(&byte_array)])?;
    Ok(obj)
}
