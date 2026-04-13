use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use automerge_jni_macros::jni_fn;
use jni::{
    errors::ThrowRuntimeExAndDefault,
    jni_sig, jni_str,
    objects::{JByteArray, JClass, JObject},
    refs::Reference,
    strings::{JNIStr, JNIString},
    sys::{jboolean, jint},
};

use crate::interop::set_array_field;

pub struct JavaObjId(automerge::ObjId);

impl AsRef<automerge::ObjId> for JavaObjId {
    fn as_ref(&self) -> &automerge::ObjId {
        &self.0
    }
}

impl From<automerge::ObjId> for JavaObjId {
    fn from(i: automerge::ObjId) -> Self {
        Self(i)
    }
}

const CLASSNAME: &JNIStr = am_classname!("ObjectId");

impl JavaObjId {
    pub(crate) fn into_jobject<'a>(
        self,
        env: &mut jni::Env<'a>,
    ) -> Result<JObject<'a>, jni::errors::Error> {
        let raw_obj = env.alloc_object(CLASSNAME)?;
        let bytes = self.0.to_bytes();
        let jbytes = env.byte_array_from_slice(&bytes)?;
        unsafe {
            set_array_field(
                env,
                &raw_obj,
                jni_str!("raw"),
                jni_sig!("[B"),
                (&jbytes).into(),
            )?
        };
        Ok(raw_obj)
    }

    pub(crate) fn from_jobject(
        env: &mut jni::Env<'_>,
        obj: JObject<'_>,
    ) -> Result<Self, jni::errors::Error> {
        if obj.is_null() {
            env.throw_new(
                jni_str!("java/lang/IllegalArgumentException"),
                jni_str!("ObjectId cannot be null"),
            )?;
            return Err(jni::errors::Error::JavaException);
        }
        let bytes_jobject = env.get_field(&obj, jni_str!("raw"), jni_sig!("[B"))?.l()?;
        let prim_array = JByteArray::cast_local(env, bytes_jobject)?;
        let bytes = env.convert_byte_array(&prim_array)?;
        match automerge::ObjId::try_from(bytes.as_slice()) {
            Ok(o) => Ok(Self(o)),
            Err(e) => {
                env.throw_new(
                    jni_str!("java/lang/IllegalArgumentException"),
                    JNIString::from(e.to_string()),
                )?;
                Err(jni::errors::Error::JavaException)
            }
        }
    }
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn rootObjectId<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
) -> JObject<'local> {
    env.with_env(|env| JavaObjId(automerge::ObjId::Root).into_jobject(env))
        .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn isRootObjectId<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    obj: JObject<'local>,
) -> bool {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj)?;
        Ok::<_, jni::errors::Error>(obj.as_ref() == &automerge::ROOT)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn objectIdToString<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    obj: JObject<'local>,
) -> JObject<'local> {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj)?;
        let s = obj.as_ref().to_string();
        let jstr = env.new_string(s)?;
        Ok::<_, jni::errors::Error>(jstr.into())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn objectIdHash<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    obj: JObject<'local>,
) -> jint {
    env.with_env(|env| {
        let obj = JavaObjId::from_jobject(env, obj)?;
        let mut hasher = DefaultHasher::new();
        obj.as_ref().hash(&mut hasher);
        Ok::<_, jni::errors::Error>(hasher.finish() as i32)
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}

#[no_mangle]
#[jni_fn]
pub unsafe extern "C" fn objectIdsEqual<'local>(
    mut env: jni::EnvUnowned<'local>,
    _class: JClass<'local>,
    left: JObject<'local>,
    right: JObject<'local>,
) -> jboolean {
    env.with_env(|env| {
        let left = JavaObjId::from_jobject(env, left)?;
        let right = JavaObjId::from_jobject(env, right)?;
        Ok::<_, jni::errors::Error>(left.as_ref() == right.as_ref())
    })
    .resolve::<ThrowRuntimeExAndDefault>()
}
