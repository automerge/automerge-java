use crate::interop::AsPointerObj;
use crate::repo::type_mappings::command_id::command_id_to_java_object;
use crate::repo::type_mappings::conn_direction::java_object_to_conn_direction;
use crate::repo::type_mappings::connection_id::java_object_to_connection_id;
use crate::repo::type_mappings::doc_to_hub_msg::java_object_to_doc_to_hub_msg;
use crate::repo::type_mappings::document_actor_id::java_object_to_document_actor_id;
use crate::repo::type_mappings::document_id::java_object_to_document_id;
use crate::repo::type_mappings::io_result::java_object_to_io_result_hub;
use automerge_jni_macros::jni_fn;
use jni::objects::{JClass, JObject, JValue};
use jni::sys::{jboolean, jint, jobject};
use jni::JNIEnv;
use samod_core::actors::hub::DispatchedCommand;
use samod_core::actors::hub::HubEvent;

impl AsPointerObj for HubEvent {
    type EnvRef<'a> = Self;
    fn classname() -> &'static str {
        am_classname!("HubEventPointer")
    }
}
