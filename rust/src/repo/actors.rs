use crate::interop::AsPointerObj;
use samod_core::actors::{document::DocumentActor, hub::Hub};
use samod_core::SamodLoader;

/// AsPointerObj implementation for samod-core Hub
impl AsPointerObj for Hub {
    type EnvRef<'a> = Self;
    fn classname() -> &'static str {
        am_classname!("AutomergeSys$HubPointer")
    }
}

/// AsPointerObj implementation for samod-core DocumentActor
impl AsPointerObj for DocumentActor {
    type EnvRef<'a> = Self;
    fn classname() -> &'static str {
        am_classname!("AutomergeSys$DocumentActorPointer")
    }
}

/// AsPointerObj implementation for samod-core SamodLoader
impl AsPointerObj for SamodLoader {
    type EnvRef<'a> = Self;
    fn classname() -> &'static str {
        am_classname!("AutomergeSys$SamodLoaderPointer")
    }
}
