use std::ops::RangeBounds;
use std::sync::MutexGuard;

use am::ReadDoc;
use jni::objects::{JObject, JObjectArray};
use jni::signature::RuntimeFieldSignature;
use jni::sys::{jint, jlong};
use jni::{jni_sig, jni_str};

use crate::am_value::{scalar_to_amvalue, to_amvalue, to_optional_amvalue};
use crate::conflicts::make_optional_conflicts;
use crate::cursor::Cursor;
use crate::interop::{
    changehash_to_jobject, heads_from_jobject, throw_amg_exc, unwrap_or_throw_amg_exc,
    CHANGEHASH_CLASS,
};
use crate::java_option::{make_empty_option, make_optional};
use crate::mark::mark_to_java;
use crate::obj_id::JavaObjId;
use crate::obj_type::JavaObjType;
use crate::prop::JProp;
use crate::{interop::JavaPointer, read_ops::ReadOps};
use automerge as am;
use automerge::transaction::OwnedTransaction;

mod cursor;
mod get;
mod get_all;
mod get_at;
mod get_object_type;
mod heads;
mod keys;
mod length;
mod list_items;
mod map_entries;
mod marks;
mod text;

pub(crate) enum SomeReadPointer<'local> {
    Doc(JObject<'local>),
    Tx(JObject<'local>),
}

impl<'local> SomeReadPointer<'local> {
    pub(crate) fn doc(obj: JObject<'local>) -> Self {
        Self::Doc(obj)
    }

    pub(crate) fn tx(obj: JObject<'local>) -> Self {
        Self::Tx(obj)
    }

    unsafe fn get<P: Into<JProp<'local>>>(
        self,
        env: &'_ mut jni::Env<'local>,
        obj: JObject<'local>,
        key: P,
    ) -> Result<JObject<'local>, jni::errors::Error> {
        let read = SomeRead::from_pointer(env, self)?;
        let obj = JavaObjId::from_jobject(env, obj)?;

        let key = key.into().try_into_prop(env)?;
        let result = unwrap_or_throw_amg_exc(env, read.get(obj, key))?;

        to_optional_amvalue(env, result)
    }

    unsafe fn get_at<P: Into<JProp<'local>>>(
        self,
        env: &'_ mut jni::Env<'local>,
        obj: JObject<'local>,
        key: P,
        heads: JObjectArray<'local>,
    ) -> Result<JObject<'local>, jni::errors::Error> {
        let read = SomeRead::from_pointer(env, self)?;
        let obj = JavaObjId::from_jobject(env, obj)?;
        let heads = heads_from_jobject(env, heads)?;

        let key = key.into().try_into_prop(env)?;
        let result = unwrap_or_throw_amg_exc(env, read.get_at(obj, key, &heads))?;

        to_optional_amvalue(env, result)
    }

    unsafe fn get_all<P: Into<JProp<'local>>>(
        self,
        env: &'_ mut jni::Env<'local>,
        obj: JObject<'local>,
        key: P,
        heads: Option<JObjectArray<'local>>,
    ) -> Result<JObject<'local>, jni::errors::Error> {
        let read = SomeRead::from_pointer(env, self)?;
        let obj = JavaObjId::from_jobject(env, obj)?;

        let key = key.into().try_into_prop(env)?;
        let heads = heads.map(|h| heads_from_jobject(env, h)).transpose()?;

        let result = unwrap_or_throw_amg_exc(
            env,
            match heads {
                Some(heads) => read.get_all_at(obj, key, &heads),
                None => read.get_all(obj, key),
            },
        )?;

        make_optional_conflicts(env, result)
    }

    unsafe fn heads(
        self,
        env: &mut jni::Env<'local>,
    ) -> Result<JObjectArray<'local>, jni::errors::Error> {
        let read = SomeRead::from_pointer(env, self)?;
        let heads = read.heads();

        let heads_arr =
            env.new_object_array(heads.len() as i32, CHANGEHASH_CLASS, JObject::null())?;
        for (i, head) in heads.iter().enumerate() {
            let hash = changehash_to_jobject(env, head)?;
            heads_arr.set_element(env, i, hash)?;
        }
        Ok(heads_arr)
    }

    unsafe fn keys(
        self,
        env: &mut jni::Env<'local>,
        obj: JObject<'local>,
        heads: Option<JObjectArray<'local>>,
    ) -> Result<JObject<'local>, jni::errors::Error> {
        let read = SomeRead::from_pointer(env, self)?;
        let obj = JavaObjId::from_jobject(env, obj)?;
        let heads = heads.map(|h| heads_from_jobject(env, h)).transpose()?;
        let keys = match unwrap_or_throw_amg_exc(env, read.object_type(&obj))? {
            automerge::ObjType::Map => match heads {
                Some(h) => read.keys_at(obj, &h).collect::<Vec<_>>(),
                None => read.keys(obj).collect::<Vec<_>>(),
            },
            _ => return make_empty_option(env),
        };
        let keys_arr = env.new_object_array(
            keys.len() as i32,
            jni_str!("java/lang/String"),
            JObject::null(),
        )?;
        for (index, k) in keys.into_iter().enumerate() {
            let k = env.new_string(k)?;
            keys_arr.set_element(env, index, k)?;
        }
        make_optional(env, (&keys_arr).into())
    }

    unsafe fn length(
        self,
        env: &mut jni::Env<'local>,
        obj: JObject<'local>,
        heads: Option<JObjectArray<'local>>,
    ) -> Result<jlong, jni::errors::Error> {
        let read = SomeRead::from_pointer(env, self)?;
        let obj = JavaObjId::from_jobject(env, obj)?;
        match heads {
            Some(h) => {
                let heads = heads_from_jobject(env, h)?;
                Ok(read.length_at(obj, &heads) as jlong)
            }
            None => Ok(read.length(obj) as jlong),
        }
    }

    unsafe fn list_items(
        self,
        env: &mut jni::Env<'local>,
        obj: JObject<'local>,
        heads: Option<JObjectArray<'local>>,
    ) -> Result<JObject<'local>, jni::errors::Error> {
        let read = SomeRead::from_pointer(env, self)?;
        let obj = JavaObjId::from_jobject(env, obj)?;
        let heads = heads.map(|h| heads_from_jobject(env, h)).transpose()?;
        let items = match read.object_type(&obj) {
            Ok(am::ObjType::List) => match heads {
                Some(h) => read.list_range_at(obj, .., &h).collect::<Vec<_>>(),
                None => read.list_range(obj, ..).collect::<Vec<_>>(),
            },
            Ok(_) | Err(am::AutomergeError::NotAnObject) => return make_empty_option(env),
            Err(e) => {
                throw_amg_exc(env, e)?;
                return Ok(JObject::null());
            }
        };

        let jitems = env.new_object_array(
            items.len() as i32,
            am_classname!("AmValue"),
            JObject::null(),
        )?;
        for (idx, item) in items.into_iter().enumerate() {
            let id = item.id();
            let val = to_amvalue(env, (item.value.into_value(), id))?;
            jitems.set_element(env, idx, val)?;
        }
        make_optional(env, (&jitems).into())
    }

    unsafe fn map_entries(
        self,
        env: &mut jni::Env<'local>,
        obj: JObject<'local>,
        heads: Option<JObjectArray<'local>>,
    ) -> Result<JObject<'local>, jni::errors::Error> {
        let read = SomeRead::from_pointer(env, self)?;
        let obj = JavaObjId::from_jobject(env, obj)?;
        let heads = heads.map(|h| heads_from_jobject(env, h)).transpose()?;

        let entries = match read.object_type(&obj) {
            Ok(automerge::ObjType::Map) => match heads {
                Some(h) => read.map_range_at(obj, .., &h).collect::<Vec<_>>(),
                None => read.map_range(obj, ..).collect::<Vec<_>>(),
            },
            Ok(..) | Err(am::AutomergeError::NotAnObject) => return make_empty_option(env),
            Err(e) => {
                throw_amg_exc(env, e)?;
                return Ok(JObject::null());
            }
        };
        let entries_arr = env.new_object_array(
            entries.len() as i32,
            am_classname!("MapEntry"),
            JObject::null(),
        )?;
        for (i, item) in entries.into_iter().enumerate() {
            let id = item.id();
            let entry = env.alloc_object(am_classname!("MapEntry"))?;
            let val = env.new_string(item.key)?;
            env.set_field(
                &entry,
                jni_str!("key"),
                jni_sig!("Ljava/lang/String;"),
                (&val).into(),
            )?;
            let am_val = to_amvalue(env, (item.value.into_value(), id))?;

            // TODO: replace with jni_sig
            let val_sig =
                RuntimeFieldSignature::from_str(format!("L{};", am_classname!("AmValue"))).unwrap();

            env.set_field(
                &entry,
                jni_str!("value"),
                val_sig.field_signature(),
                (&am_val).into(),
            )?;
            entries_arr.set_element(env, i, entry)?;
        }
        make_optional(env, (&entries_arr).into())
    }

    unsafe fn text(
        self,
        env: &mut jni::Env<'local>,
        obj: JObject<'local>,
        heads: Option<JObjectArray<'local>>,
    ) -> Result<JObject<'local>, jni::errors::Error> {
        let read = SomeRead::from_pointer(env, self)?;
        let obj = JavaObjId::from_jobject(env, obj)?;
        let heads = heads.map(|h| heads_from_jobject(env, h)).transpose()?;
        let text = match read.object_type(&obj) {
            Ok(am::ObjType::Text) => match heads {
                Some(h) => read.text_at(obj, &h),
                None => read.text(obj),
            },
            Ok(..) | Err(am::AutomergeError::NotAnObject) => {
                return make_empty_option(env);
            }
            Err(e) => {
                throw_amg_exc(env, e)?;
                return Ok(JObject::null());
            }
        };
        let text = env.new_string(unwrap_or_throw_amg_exc(env, text)?)?;
        make_optional(env, (&text).into())
    }

    unsafe fn marks(
        self,
        env: &mut jni::Env<'local>,
        obj: JObject<'local>,
        heads_option: JObject<'local>,
    ) -> Result<JObject<'local>, jni::errors::Error> {
        let read = SomeRead::from_pointer(env, self)?;
        let obj = JavaObjId::from_jobject(env, obj)?;
        let heads = maybe_heads(env, heads_option)?;
        let marks = if let Some(h) = heads {
            read.marks_at(obj, &h)
        } else {
            read.marks(obj)
        };
        let marks = unwrap_or_throw_amg_exc(env, marks)?;
        let marks_arr = env.new_object(jni_str!("java/util/ArrayList"), jni_sig!("()V"), &[])?;
        for mark in marks {
            let jmark = mark_to_java(env, &mark)?;
            env.call_method(
                &marks_arr,
                jni_str!("add"),
                jni_sig!("(Ljava/lang/Object;)Z"),
                &[(&jmark).into()],
            )?;
        }
        Ok(marks_arr)
    }

    unsafe fn marks_at_index(
        self,
        env: &mut jni::Env<'local>,
        obj: JObject<'local>,
        index: jint,
        heads_option: JObject<'local>,
    ) -> Result<JObject<'local>, jni::errors::Error> {
        let read = SomeRead::from_pointer(env, self)?;
        let obj = JavaObjId::from_jobject(env, obj)?;
        let heads = maybe_heads(env, heads_option)?;
        let marks = if let Some(h) = heads {
            read.get_marks(obj, index as usize, Some(&h))
        } else {
            read.get_marks(obj, index as usize, None)
        };
        let marks = unwrap_or_throw_amg_exc(env, marks)?;
        let marks_map = env.new_object(jni_str!("java/util/HashMap"), jni_sig!("()V"), &[])?;
        for (mark_name, mark_value) in marks.iter() {
            let value = scalar_to_amvalue(env, mark_value)?;
            let mark_name = env.new_string(mark_name)?;
            env.call_method(
                &marks_map,
                jni_str!("put"),
                jni_sig!("(Ljava/lang/Object;Ljava/lang/Object;)Ljava/lang/Object;"),
                &[(&mark_name).into(), (&value).into()],
            )?;
        }
        Ok(marks_map)
    }

    unsafe fn make_cursor(
        self,
        env: &mut jni::Env<'local>,
        obj: JObject<'local>,
        index: jlong,
        heads: JObject<'local>,
    ) -> Result<JObject<'local>, jni::errors::Error> {
        let obj = JavaObjId::from_jobject(env, obj)?;
        let heads = maybe_heads(env, heads)?;
        let read = SomeRead::from_pointer(env, self)?;
        if index < 0 {
            env.throw_new(
                jni_str!("java/lang/IllegalArgumentException"),
                jni_str!("Index must be >= 0"),
            )?;
            return Ok(JObject::null());
        }
        let cursor =
            unwrap_or_throw_amg_exc(env, read.get_cursor(obj, index as usize, heads.as_deref()))?;
        Cursor::from(cursor).into_jobject(env)
    }

    unsafe fn lookup_cursor_index(
        self,
        env: &mut jni::Env<'local>,
        obj: JObject<'local>,
        cursor: JObject<'local>,
        heads: JObject<'local>,
    ) -> Result<jlong, jni::errors::Error> {
        let obj = JavaObjId::from_jobject(env, obj)?;
        let heads = maybe_heads(env, heads)?;
        let read = SomeRead::from_pointer(env, self)?;

        let cursor = Cursor::from_jobject(env, cursor)?;
        let index = unwrap_or_throw_amg_exc(
            env,
            read.get_cursor_position(obj, cursor.as_ref(), heads.as_deref()),
        )?;
        Ok(index as i64)
    }

    unsafe fn get_object_type(
        self,
        env: &mut jni::Env<'local>,
        obj: JObject<'local>,
    ) -> Result<JObject<'local>, jni::errors::Error> {
        let obj = JavaObjId::from_jobject(env, obj)?;
        let read = SomeRead::from_pointer(env, self)?;
        let obj_type = match read.object_type(obj) {
            Ok(o) => o,
            Err(automerge::AutomergeError::InvalidObjId(_)) => {
                return make_empty_option(env);
            }
            Err(e) => {
                throw_amg_exc(env, e)?;
                return Ok(JObject::null());
            }
        };
        let val = JavaObjType::from(obj_type).to_java_enum(env)?;
        make_optional(env, (&val).into())
    }
}

unsafe fn maybe_heads<'local>(
    env: &mut jni::Env<'local>,
    maybe_heads: JObject<'local>,
) -> Result<Option<Vec<automerge::ChangeHash>>, jni::errors::Error> {
    let heads_present =
        env.call_method(&maybe_heads, jni_str!("isPresent"), jni_sig!("()Z"), &[])?;
    if heads_present.z()? {
        let heads = env
            .call_method(
                maybe_heads,
                jni_str!("get"),
                jni_sig!("()Ljava/lang/Object;"),
                &[],
            )?
            .l()?;
        let heads = JObjectArray::<JObject>::cast_local(env, heads)?;
        Ok(Some(heads_from_jobject(env, heads)?))
    } else {
        Ok(None)
    }
}

// Existential type over all implementations of ReadOps
enum SomeRead<'a> {
    Transaction(MutexGuard<'a, OwnedTransaction>),
    Doc(MutexGuard<'a, automerge::Automerge>),
}

impl<'local> SomeRead<'local> {
    unsafe fn from_pointer(
        env: &'_ mut jni::Env<'local>,
        pointer: SomeReadPointer<'local>,
    ) -> Result<SomeRead<'local>, jni::errors::Error> {
        match pointer {
            SomeReadPointer::Doc(doc_pointer) => Self::from_doc_pointer(env, doc_pointer),
            SomeReadPointer::Tx(tx) => Self::from_tx_pointer(env, tx),
        }
    }

    pub(crate) unsafe fn from_tx_pointer(
        env: &'_ mut jni::Env<'local>,
        pointer: JObject<'local>,
    ) -> Result<SomeRead<'local>, jni::errors::Error> {
        let tx = OwnedTransaction::borrow_from_pointer(env, pointer)?;
        Ok(Self::Transaction(tx))
    }

    pub(crate) unsafe fn from_doc_pointer(
        env: &'_ mut jni::Env<'local>,
        pointer: JObject<'local>,
    ) -> Result<SomeRead<'local>, jni::errors::Error> {
        let am = automerge::Automerge::borrow_from_pointer(env, pointer)?;
        Ok(SomeRead::Doc(am))
    }
}

impl<'a> ReadDoc for SomeRead<'a> {
    fn get<O: AsRef<am::ObjId>, P: Into<am::Prop>>(
        &self,
        obj: O,
        prop: P,
    ) -> Result<Option<(am::Value<'_>, am::ObjId)>, am::AutomergeError> {
        match self {
            SomeRead::Transaction(tx) => tx.get(obj, prop),
            SomeRead::Doc(doc) => doc.get(obj, prop),
        }
    }

    fn get_at<O: AsRef<am::ObjId>, P: Into<am::Prop>>(
        &self,
        obj: O,
        prop: P,
        heads: &[am::ChangeHash],
    ) -> Result<Option<(am::Value<'_>, am::ObjId)>, am::AutomergeError> {
        match self {
            SomeRead::Transaction(tx) => tx.get_at(obj, prop, heads),
            SomeRead::Doc(doc) => doc.get_at(obj, prop, heads),
        }
    }

    fn get_all<O: AsRef<am::ObjId>, P: Into<am::Prop>>(
        &self,
        obj: O,
        prop: P,
    ) -> Result<Vec<(am::Value<'_>, am::ObjId)>, am::AutomergeError> {
        match self {
            SomeRead::Transaction(tx) => tx.get_all(obj, prop),
            SomeRead::Doc(doc) => doc.get_all(obj, prop),
        }
    }

    fn get_all_at<O: AsRef<am::ObjId>, P: Into<am::Prop>>(
        &self,
        obj: O,
        prop: P,
        heads: &[am::ChangeHash],
    ) -> Result<Vec<(am::Value<'_>, am::ObjId)>, am::AutomergeError> {
        match self {
            SomeRead::Transaction(tx) => tx.get_all_at(obj, prop, heads),
            SomeRead::Doc(doc) => doc.get_all_at(obj, prop, heads),
        }
    }

    fn keys<O: AsRef<am::ObjId>>(&self, obj: O) -> am::iter::Keys<'_> {
        match self {
            SomeRead::Transaction(tx) => tx.keys(obj),
            SomeRead::Doc(doc) => doc.keys(obj),
        }
    }

    fn keys_at<O: AsRef<am::ObjId>>(&self, obj: O, heads: &[am::ChangeHash]) -> am::iter::Keys<'_> {
        match self {
            SomeRead::Transaction(tx) => tx.keys_at(obj, heads),
            SomeRead::Doc(doc) => doc.keys_at(obj, heads),
        }
    }

    fn object_type<O: AsRef<am::ObjId>>(&self, obj: O) -> Result<am::ObjType, am::AutomergeError> {
        match self {
            SomeRead::Transaction(tx) => tx.object_type(obj),
            SomeRead::Doc(doc) => doc.object_type(obj),
        }
    }

    fn map_range<'b, O: AsRef<am::ObjId>, R: RangeBounds<String> + 'b>(
        &'b self,
        obj: O,
        range: R,
    ) -> am::iter::MapRange<'b> {
        match self {
            SomeRead::Transaction(tx) => tx.map_range(obj, range),
            SomeRead::Doc(doc) => doc.map_range(obj, range),
        }
    }

    fn map_range_at<'b, O: AsRef<am::ObjId>, R: RangeBounds<String> + 'b>(
        &'b self,
        obj: O,
        range: R,
        heads: &[am::ChangeHash],
    ) -> am::iter::MapRange<'b> {
        match self {
            SomeRead::Transaction(tx) => tx.map_range_at(obj, range, heads),
            SomeRead::Doc(doc) => doc.map_range_at(obj, range, heads),
        }
    }

    fn list_range<O: AsRef<am::ObjId>, R: std::ops::RangeBounds<usize>>(
        &self,
        obj: O,
        range: R,
    ) -> am::iter::ListRange<'_> {
        match self {
            SomeRead::Transaction(tx) => tx.list_range(obj, range),
            SomeRead::Doc(doc) => doc.list_range(obj, range),
        }
    }

    fn list_range_at<O: AsRef<am::ObjId>, R: std::ops::RangeBounds<usize>>(
        &self,
        obj: O,
        range: R,
        heads: &[am::ChangeHash],
    ) -> am::iter::ListRange<'_> {
        match self {
            SomeRead::Transaction(tx) => tx.list_range_at(obj, range, heads),
            SomeRead::Doc(doc) => doc.list_range_at(obj, range, heads),
        }
    }

    fn length<O: AsRef<am::ObjId>>(&self, obj: O) -> usize {
        match self {
            SomeRead::Transaction(tx) => tx.length(obj),
            SomeRead::Doc(doc) => doc.length(obj),
        }
    }

    fn length_at<O: AsRef<am::ObjId>>(&self, obj: O, heads: &[am::ChangeHash]) -> usize {
        match self {
            SomeRead::Transaction(tx) => tx.length_at(obj, heads),
            SomeRead::Doc(doc) => doc.length_at(obj, heads),
        }
    }

    fn text<O: AsRef<am::ObjId>>(&self, obj: O) -> Result<String, am::AutomergeError> {
        match self {
            SomeRead::Transaction(tx) => tx.text(obj),
            SomeRead::Doc(doc) => doc.text(obj),
        }
    }

    fn text_at<O: AsRef<am::ObjId>>(
        &self,
        obj: O,
        heads: &[am::ChangeHash],
    ) -> Result<String, am::AutomergeError> {
        match self {
            SomeRead::Transaction(tx) => tx.text_at(obj, heads),
            SomeRead::Doc(doc) => doc.text_at(obj, heads),
        }
    }

    fn parents<O: AsRef<am::ObjId>>(&self, obj: O) -> Result<am::Parents<'_>, am::AutomergeError> {
        match self {
            SomeRead::Transaction(tx) => tx.parents(obj),
            SomeRead::Doc(doc) => doc.parents(obj),
        }
    }

    fn parents_at<O: AsRef<am::ObjId>>(
        &self,
        obj: O,
        heads: &[am::ChangeHash],
    ) -> Result<am::Parents<'_>, am::AutomergeError> {
        match self {
            SomeRead::Transaction(tx) => tx.parents_at(obj, heads),
            SomeRead::Doc(doc) => doc.parents_at(obj, heads),
        }
    }

    fn values<O: AsRef<am::ObjId>>(&self, obj: O) -> am::iter::Values<'_> {
        match self {
            SomeRead::Transaction(tx) => tx.values(obj),
            SomeRead::Doc(doc) => doc.values(obj),
        }
    }

    fn values_at<O: AsRef<am::ObjId>>(
        &self,
        obj: O,
        heads: &[am::ChangeHash],
    ) -> am::iter::Values<'_> {
        match self {
            SomeRead::Transaction(tx) => tx.values_at(obj, heads),
            SomeRead::Doc(doc) => doc.values_at(obj, heads),
        }
    }

    fn get_missing_deps(&self, heads: &[am::ChangeHash]) -> Vec<am::ChangeHash> {
        match self {
            SomeRead::Transaction(tx) => tx.get_missing_deps(heads),
            SomeRead::Doc(doc) => doc.get_missing_deps(heads),
        }
    }

    fn get_change_by_hash(&self, hash: &am::ChangeHash) -> Option<am::Change> {
        match self {
            SomeRead::Transaction(tx) => tx.get_change_by_hash(hash),
            SomeRead::Doc(doc) => doc.get_change_by_hash(hash),
        }
    }

    fn marks<O: AsRef<am::ObjId>>(
        &self,
        obj: O,
    ) -> Result<Vec<am::marks::Mark>, am::AutomergeError> {
        match self {
            SomeRead::Transaction(tx) => tx.marks(obj),
            SomeRead::Doc(doc) => doc.marks(obj),
        }
    }

    fn marks_at<O: AsRef<am::ObjId>>(
        &self,
        obj: O,
        heads: &[am::ChangeHash],
    ) -> Result<Vec<am::marks::Mark>, am::AutomergeError> {
        match self {
            SomeRead::Transaction(tx) => tx.marks_at(obj, heads),
            SomeRead::Doc(doc) => doc.marks_at(obj, heads),
        }
    }

    fn get_cursor<O: AsRef<am::ObjId>, I: Into<am::CursorPosition>>(
        &self,
        obj: O,
        position: I,
        at: Option<&[am::ChangeHash]>,
    ) -> Result<am::Cursor, am::AutomergeError> {
        match self {
            SomeRead::Transaction(tx) => tx.get_cursor(obj, position, at),
            SomeRead::Doc(doc) => doc.get_cursor(obj, position, at),
        }
    }

    fn get_cursor_position<O: AsRef<am::ObjId>>(
        &self,
        obj: O,
        cursor: &am::Cursor,
        at: Option<&[am::ChangeHash]>,
    ) -> Result<usize, am::AutomergeError> {
        match self {
            SomeRead::Transaction(tx) => tx.get_cursor_position(obj, cursor, at),
            SomeRead::Doc(doc) => doc.get_cursor_position(obj, cursor, at),
        }
    }

    fn get_marks<O: AsRef<am::ObjId>>(
        &self,
        obj: O,
        index: usize,
        heads: Option<&[am::ChangeHash]>,
    ) -> Result<am::marks::MarkSet, am::AutomergeError> {
        match self {
            SomeRead::Transaction(tx) => tx.get_marks(obj, index, heads),
            SomeRead::Doc(doc) => doc.get_marks(obj, index, heads),
        }
    }

    fn iter_at<O: AsRef<automerge::ObjId>>(
        &self,
        obj: O,
        heads: Option<&[automerge::ChangeHash]>,
    ) -> automerge::iter::DocIter<'_> {
        match self {
            SomeRead::Transaction(tx) => tx.iter_at(obj, heads),
            SomeRead::Doc(doc) => doc.iter_at(obj, heads),
        }
    }

    fn spans<O: AsRef<automerge::ObjId>>(
        &self,
        obj: O,
    ) -> Result<automerge::iter::Spans<'_>, automerge::AutomergeError> {
        match self {
            SomeRead::Transaction(tx) => tx.spans(obj),
            SomeRead::Doc(doc) => doc.spans(obj),
        }
    }

    fn spans_at<O: AsRef<automerge::ObjId>>(
        &self,
        obj: O,
        heads: &[automerge::ChangeHash],
    ) -> Result<automerge::iter::Spans<'_>, automerge::AutomergeError> {
        match self {
            SomeRead::Transaction(tx) => tx.spans_at(obj, heads),
            SomeRead::Doc(doc) => doc.spans_at(obj, heads),
        }
    }

    fn get_cursor_moving<O: AsRef<automerge::ObjId>, I: Into<automerge::CursorPosition>>(
        &self,
        obj: O,
        position: I,
        at: Option<&[automerge::ChangeHash]>,
        move_cursor: automerge::MoveCursor,
    ) -> Result<automerge::Cursor, automerge::AutomergeError> {
        match self {
            SomeRead::Transaction(tx) => tx.get_cursor_moving(obj, position, at, move_cursor),
            SomeRead::Doc(doc) => doc.get_cursor_moving(obj, position, at, move_cursor),
        }
    }

    fn hydrate<O: AsRef<automerge::ObjId>>(
        &self,
        obj: O,
        heads: Option<&[automerge::ChangeHash]>,
    ) -> Result<automerge::hydrate::Value, automerge::AutomergeError> {
        match self {
            SomeRead::Transaction(tx) => tx.hydrate(obj, heads),
            SomeRead::Doc(doc) => ReadDoc::hydrate(&**doc, obj, heads),
        }
    }

    fn stats(&self) -> automerge::Stats {
        match self {
            SomeRead::Transaction(tx) => tx.stats(),
            SomeRead::Doc(doc) => doc.stats(),
        }
    }

    fn text_encoding(&self) -> automerge::TextEncoding {
        match self {
            SomeRead::Transaction(tx) => tx.text_encoding(),
            SomeRead::Doc(doc) => doc.text_encoding(),
        }
    }
}

impl<'a> ReadOps for SomeRead<'a> {
    fn heads(&self) -> Vec<am::ChangeHash> {
        match self {
            SomeRead::Transaction(tx) => tx.get_heads(),
            SomeRead::Doc(doc) => doc.heads(),
        }
    }
}
