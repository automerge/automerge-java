use crate::{interop::AsPointerObj, repo::type_mappings::io_task::io_task_storage_to_java_object};
use jni::{
    objects::{JObject, JValue},
    JNIEnv,
};
use samod_core::LoaderState;

/// Convert a Rust LoaderState to a Java LoaderStepResult object
///
/// This function converts the enum variants of LoaderState to the corresponding
/// Java LoaderStepResult subclasses.
///
/// # Arguments
/// * `env` - JNI environment
/// * `loader_state` - The Rust LoaderState enum to convert
///
/// # Returns
/// A Java LoaderStepResult object (either NeedIo or Loaded variant)
pub(crate) fn loader_state_to_java_object<'local>(
    env: &mut JNIEnv<'local>,
    loader_state: LoaderState,
) -> Result<JObject<'local>, jni::errors::Error> {
    match loader_state {
        LoaderState::NeedIo(io_tasks) => {
            // Convert Vec<IoTask<StorageTask>> to Java IoTask[]
            let task_objects: Vec<_> = io_tasks
                .iter()
                .map(|task| io_task_storage_to_java_object(env, task))
                .collect::<Result<Vec<_>, _>>()?;

            // Create Java IoTask array
            let io_task_class = env.find_class(am_classname!("IoTask"))?;
            let task_array =
                env.new_object_array(task_objects.len() as i32, &io_task_class, JObject::null())?;

            // Fill the array with IoTask objects
            for (i, task_obj) in task_objects.into_iter().enumerate() {
                env.set_object_array_element(&task_array, i as i32, task_obj)?;
            }

            env.new_object(
                am_classname!("LoaderStepResult$NeedIo"),
                format!("([L{};)V", am_classname!("IoTask")),
                &[JValue::from(&task_array)],
            )
        }
        LoaderState::Loaded(hub) => {
            // Convert Box<Hub> to HubPointer
            let hub_obj = match (*hub).to_pointer_obj(env) {
                Ok(ptr) => ptr,
                Err(_) => return Err(jni::errors::Error::JavaException),
            };

            env.new_object(
                am_classname!("LoaderStepResult$Loaded"),
                format!("(L{};)V", am_classname!("AutomergeSys$HubPointer")),
                &[JValue::from(&hub_obj)],
            )
        }
    }
}
