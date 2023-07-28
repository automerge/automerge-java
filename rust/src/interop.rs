use am::PatchLog;
use automerge::{self as am, transaction::Transaction, ChangeHash};
use jni::{
    objects::{JObject, JValue},
    sys::jobject,
};

pub(crate) const CHANGEHASH_CLASS: &str = am_classname!("ChangeHash");

/// A trait for objects which are represented in Java land as a pointer wrapper
pub(crate) trait AsPointerObj: Sized {
    type EnvRef<'a>;
    /// Fully qualified classname of the pointer type
    fn classname() -> &'static str;

    fn from_pointer_obj<'a>(
        env: &jni::JNIEnv<'a>,
        obj: jobject,
    ) -> Result<&'a mut Self::EnvRef<'a>, errors::FromPointerObj> {
        let obj = unsafe { JObject::from_raw(obj) };
        let raw_pointer = env
            .get_field(obj, "pointer", "J")
            .map_err(errors::FromPointerObj::GetPointer)?
            .j()
            .map_err(errors::FromPointerObj::ConvertToI64)?;
        let result = unsafe { &mut *(raw_pointer as *mut Self::EnvRef<'a>) };
        Ok(result)
    }

    fn owned_from_pointer_obj<'b>(
        env: &jni::JNIEnv<'b>,
        obj: jobject,
    ) -> Result<Box<Self::EnvRef<'b>>, errors::FromPointerObj> {
        let obj = unsafe { JObject::from_raw(obj) };
        let raw_pointer = env
            .get_field(obj, "pointer", "J")
            .map_err(errors::FromPointerObj::GetPointer)?
            .j()
            .map_err(errors::FromPointerObj::ConvertToI64)?;
        let result = unsafe { Box::from_raw(raw_pointer as *mut Self::EnvRef<'b>) };
        Ok(result)
    }

    fn to_pointer_obj(self, env: &jni::JNIEnv) -> Result<jobject, errors::ConstructPointerObj> {
        let boxed = Box::new(self);
        let ptr = JValue::from(Box::into_raw(boxed) as i64);
        let obj = env.alloc_object(Self::classname()).map_err(|e| {
            errors::ConstructPointerObj::Alloc {
                classname: Self::classname(),
                err: e,
            }
        })?;
        env.set_field(obj, "pointer", "J", ptr).map_err(|e| {
            errors::ConstructPointerObj::SetPointer {
                classname: Self::classname(),
                err: e,
            }
        })?;
        Ok(obj.into_raw())
    }
}

impl AsPointerObj for automerge::Automerge {
    type EnvRef<'a> = automerge::Automerge;
    fn classname() -> &'static str {
        am_classname!("AutomergeSys$DocPointer")
    }
}

impl<'a> AsPointerObj for automerge::transaction::Transaction<'a> {
    type EnvRef<'b> = Transaction<'a>;
    fn classname() -> &'static str {
        am_classname!("AutomergeSys$TransactionPointer")
    }
}

impl AsPointerObj for automerge::sync::State {
    type EnvRef<'a> = automerge::sync::State;
    fn classname() -> &'static str {
        am_classname!("AutomergeSys$SyncStatePointer")
    }
}

impl AsPointerObj for PatchLog {
    type EnvRef<'a> = am::patches::PatchLog;
    fn classname() -> &'static str {
        am_classname!("AutomergeSys$PatchLogPointer")
    }
}

pub(crate) mod errors {
    #[derive(Debug, thiserror::Error)]
    pub(crate) enum FromPointerObj {
        #[error("unable to get the 'pointer' field from the jobject: {0}")]
        GetPointer(jni::errors::Error),
        #[error("unable to convert the 'pointer' field to an i64: {0}")]
        ConvertToI64(jni::errors::Error),
    }

    #[derive(Debug, thiserror::Error)]
    pub(crate) enum ConstructPointerObj {
        #[error("unable to alloc object of type {classname}: {err}")]
        Alloc {
            classname: &'static str,
            err: jni::errors::Error,
        },
        #[error("unable to set the 'pointer' field for {classname}: {err}")]
        SetPointer {
            classname: &'static str,
            err: jni::errors::Error,
        },
    }
}

/// Given a pointer to an array of java ChangeHash objects, return a vector of ChangeHashes.
pub(crate) unsafe fn heads_from_jobject(
    env: &jni::JNIEnv,
    heads_pointer: jobject,
) -> Result<Vec<ChangeHash>, jni::errors::Error> {
    let head_len = env.get_array_length(heads_pointer)?;
    let mut heads = Vec::with_capacity(head_len as usize);
    for i in 0..head_len {
        let head_obj = env.get_object_array_element(heads_pointer, i).unwrap();
        let head_bytes_ref = env.get_field(head_obj, "hash", "[B").unwrap().l().unwrap();
        let head_bytes = env.convert_byte_array(head_bytes_ref.into_raw()).unwrap();
        heads.push(automerge::ChangeHash::try_from(head_bytes.as_slice()).unwrap());
    }
    Ok(heads)
}

pub(crate) unsafe fn changehash_to_jobject<'a>(
    env: &jni::JNIEnv<'a>,
    hash: &ChangeHash,
) -> Result<JObject<'a>, jni::errors::Error> {
    let jhash = env.alloc_object(CHANGEHASH_CLASS)?;
    let byte_array = JObject::from_raw(env.byte_array_from_slice(hash.as_ref())?);
    env.set_field(jhash, "hash", "[B", byte_array.into())
        .unwrap();
    Ok(jhash)
}

/// Rust objects which can be converted to a java object
pub(crate) trait ToJniObject {
    fn to_jni_object<'a>(self, env: &jni::JNIEnv<'a>) -> Result<JObject<'a>, jni::errors::Error>;
}

impl ToJniObject for ChangeHash {
    fn to_jni_object<'b>(self, env: &jni::JNIEnv<'b>) -> Result<JObject<'b>, jni::errors::Error> {
        unsafe { changehash_to_jobject(env, &self) }
    }
}
