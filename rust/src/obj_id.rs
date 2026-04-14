use std::{
    collections::hash_map::DefaultHasher,
    hash::{Hash, Hasher},
};

use jni::{
    jni_str,
    objects::{JClass, JObject, JString},
    strings::JNIString,
    sys::{jboolean, jint},
    NativeMethod,
};

use crate::bindings;

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

impl JavaObjId {
    pub(crate) fn into_object_id<'a>(
        self,
        env: &mut jni::Env<'a>,
    ) -> Result<bindings::ObjectId<'a>, jni::errors::Error> {
        let bytes = self.0.to_bytes();
        let jbytes = env.byte_array_from_slice(&bytes)?;
        bindings::ObjectId::new(env, &jbytes)
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
        let oid = bindings::ObjectId::cast_local(env, obj)?;
        let jbytes = oid.raw(env)?;
        let bytes = env.convert_byte_array(&jbytes)?;
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

    pub(crate) fn from_object_id(
        env: &mut jni::Env<'_>,
        oid: bindings::ObjectId<'_>,
    ) -> Result<Self, jni::errors::Error> {
        Self::from_jobject(env, oid.into())
    }
}

const _METHODS: &[NativeMethod] = &[
    ams_native! { static extern fn root_object_id() -> bindings::ObjectId },
    ams_native! { static extern fn is_root_object_id(obj: bindings::ObjectId) -> jboolean },
    ams_native! { static extern fn object_id_to_string(obj: bindings::ObjectId) -> JString },
    ams_native! { static extern fn object_id_hash(obj: bindings::ObjectId) -> jint },
    ams_native! { static extern fn object_ids_equal(left: bindings::ObjectId, right: bindings::ObjectId) -> jboolean },
];

fn root_object_id<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
) -> jni::errors::Result<bindings::ObjectId<'local>> {
    JavaObjId(automerge::ObjId::Root).into_object_id(env)
}

fn is_root_object_id<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    obj: bindings::ObjectId<'local>,
) -> jni::errors::Result<jboolean> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    Ok(obj.as_ref() == &automerge::ROOT )
}

fn object_id_to_string<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    obj: bindings::ObjectId<'local>,
) -> jni::errors::Result<JString<'local>> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    env.new_string(obj.as_ref().to_string())
}

fn object_id_hash<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    obj: bindings::ObjectId<'local>,
) -> jni::errors::Result<jint> {
    let obj = JavaObjId::from_object_id(env, obj)?;
    let mut hasher = DefaultHasher::new();
    obj.as_ref().hash(&mut hasher);
    Ok(hasher.finish() as i32)
}

fn object_ids_equal<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    left: bindings::ObjectId<'local>,
    right: bindings::ObjectId<'local>,
) -> jni::errors::Result<jboolean> {
    let left = JavaObjId::from_object_id(env, left)?;
    let right = JavaObjId::from_object_id(env, right)?;
    Ok(left.as_ref() == right.as_ref() )
}
