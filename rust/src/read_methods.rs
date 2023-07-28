use std::ops::RangeBounds;

use am::iter::{ListRangeItem, MapRangeItem};
use am::ReadDoc;
use jni::objects::JObject;
use jni::sys::{jlong, jobject};

use crate::am_value::{to_amvalue, to_optional_amvalue};
use crate::conflicts::make_optional_conflicts;
use crate::interop::{changehash_to_jobject, heads_from_jobject, CHANGEHASH_CLASS};
use crate::java_option::{make_empty_option, make_optional};
use crate::mark::mark_to_java;
use crate::obj_id::JavaObjId;
use crate::prop::JProp;
use crate::AUTOMERGE_EXCEPTION;
use crate::{interop::AsPointerObj, read_ops::ReadOps};
use automerge as am;
use automerge::transaction::Transaction as AmTransaction;

mod get;
mod get_all;
mod get_at;
mod heads;
mod keys;
mod length;
mod list_items;
mod map_entries;
mod marks;
mod text;

macro_rules! catch {
    ($env:ident, $e:expr) => {
        match $e {
            Ok(r) => r,
            Err(e) => {
                $env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
                return JObject::null().into_raw();
            }
        }
    };
}

pub(crate) enum SomeReadPointer {
    Doc(jobject),
    Tx(jobject),
}

impl SomeReadPointer {
    pub(crate) fn doc(obj: jobject) -> Self {
        Self::Doc(obj)
    }

    pub(crate) fn tx(obj: jobject) -> Self {
        Self::Tx(obj)
    }

    unsafe fn get<'a, P: Into<JProp<'a>>>(
        self,
        env: jni::JNIEnv<'a>,
        obj_pointer: jobject,
        key: P,
    ) -> jobject {
        let read = SomeRead::from_pointer(env, self);
        let obj = JavaObjId::from_raw(&env, obj_pointer).unwrap();

        let key = catch!(env, key.into().try_into_prop(env));
        let result = catch!(env, read.get(obj, key));

        to_optional_amvalue(&env, result).unwrap().into_raw()
    }

    unsafe fn get_at<'a, P: Into<JProp<'a>>>(
        self,
        env: jni::JNIEnv<'a>,
        obj_pointer: jobject,
        key: P,
        heads_pointer: jobject,
    ) -> jobject {
        let read = SomeRead::from_pointer(env, self);
        let obj = JavaObjId::from_raw(&env, obj_pointer).unwrap();
        let heads = heads_from_jobject(&env, heads_pointer).unwrap();

        let key = catch!(env, key.into().try_into_prop(env));
        let result = catch!(env, read.get_at(obj, key, &heads));

        to_optional_amvalue(&env, result).unwrap().into_raw()
    }

    unsafe fn get_all<'a, P: Into<JProp<'a>>>(
        self,
        env: jni::JNIEnv<'a>,
        obj_pointer: jobject,
        key: P,
        heads: Option<jobject>,
    ) -> jobject {
        let read = SomeRead::from_pointer(env, self);
        let obj = JavaObjId::from_raw(&env, obj_pointer).unwrap();

        let key = catch!(env, key.into().try_into_prop(env));
        let heads = heads.map(|h| heads_from_jobject(&env, h).unwrap());

        use automerge::Prop;
        let result: Result<_, automerge::AutomergeError> =
            || -> Result<_, automerge::AutomergeError> {
                match (&key, read.object_type(&obj)?) {
                    (Prop::Map(_), automerge::ObjType::Map) => {
                        let value = match heads {
                            Some(heads) => read.get_all_at(obj, key, &heads)?,
                            None => read.get_all(obj, key)?,
                        };
                        Ok(make_optional_conflicts(env, value))
                    }
                    (Prop::Seq(_), automerge::ObjType::List | automerge::ObjType::Text) => {
                        let values = match heads {
                            Some(heads) => read.get_all_at(obj, key, &heads)?,
                            None => read.get_all(obj, key)?,
                        };
                        Ok(make_optional_conflicts(env, values))
                    }
                    _ => Ok(None),
                }
            }();

        match result {
            Ok(Some(c)) => make_optional(&env, c.into()).unwrap().into_raw(),
            Ok(None) => make_empty_option(&env).unwrap().into_raw(),
            Err(e) => {
                env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
                JObject::null().into_raw()
            }
        }
    }

    unsafe fn heads(self, env: jni::JNIEnv) -> jobject {
        let read = SomeRead::from_pointer(env, self);
        let heads = read.heads();

        let heads_arr = env
            .new_object_array(heads.len() as i32, CHANGEHASH_CLASS, JObject::null())
            .unwrap();
        for (i, head) in heads.iter().enumerate() {
            let hash = changehash_to_jobject(&env, head).unwrap();
            env.set_object_array_element(heads_arr, i as i32, hash)
                .unwrap();
        }
        heads_arr
    }

    unsafe fn keys(
        self,
        env: jni::JNIEnv,
        obj_pointer: jobject,
        heads: Option<jobject>,
    ) -> jobject {
        let read = SomeRead::from_pointer(env, self);
        let obj = JavaObjId::from_raw(&env, obj_pointer).unwrap();
        let heads = heads.map(|h| heads_from_jobject(&env, h).unwrap());
        let keys = match read.object_type(&obj) {
            Ok(automerge::ObjType::Map) => match heads {
                Some(h) => read.keys_at(obj, &h).collect::<Vec<_>>(),
                None => read.keys(obj).collect::<Vec<_>>(),
            },
            Ok(_) => return make_empty_option(&env).unwrap().into_raw(),
            Err(e) => {
                env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
                return JObject::null().into_raw();
            }
        };
        let keys_arr = env
            .new_object_array(keys.len() as i32, "java/lang/String", JObject::null())
            .unwrap();
        for (index, k) in keys.into_iter().enumerate() {
            let k = env.new_string(k).unwrap();
            env.set_object_array_element(keys_arr, index as i32, k)
                .unwrap();
        }
        let arr_obj = JObject::from_raw(keys_arr);
        make_optional(&env, arr_obj.into()).unwrap().into_raw()
    }

    unsafe fn length(
        self,
        env: jni::JNIEnv,
        obj_pointer: jobject,
        heads: Option<jobject>,
    ) -> jlong {
        let read = SomeRead::from_pointer(env, self);
        let obj = JavaObjId::from_raw(&env, obj_pointer).unwrap();
        match heads {
            Some(h) => {
                let heads = heads_from_jobject(&env, h).unwrap();
                read.length_at(obj, &heads) as i64
            }
            None => read.length(obj) as i64,
        }
    }

    unsafe fn list_items(
        self,
        env: jni::JNIEnv,
        obj_pointer: jobject,
        heads: Option<jobject>,
    ) -> jobject {
        let read = SomeRead::from_pointer(env, self);
        let obj = JavaObjId::from_raw(&env, obj_pointer).unwrap();
        let heads = heads.map(|h| heads_from_jobject(&env, h).unwrap());
        let items = match read.object_type(&obj) {
            Ok(am::ObjType::List) => match heads {
                Some(h) => read.list_range_at(obj, .., &h).collect::<Vec<_>>(),
                None => read.list_range(obj, ..).collect::<Vec<_>>(),
            },
            Ok(_) => return make_empty_option(&env).unwrap().into_raw(),
            Err(am::AutomergeError::NotAnObject) => {
                return make_empty_option(&env).unwrap().into_raw()
            }
            Err(e) => {
                env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
                return JObject::null().into_raw();
            }
        };

        let jitems = env
            .new_object_array(
                items.len() as i32,
                am_classname!("AmValue"),
                JObject::null(),
            )
            .unwrap();
        for (idx, ListRangeItem { value, id, .. }) in items.into_iter().enumerate() {
            let val = to_amvalue(&env, (value, id)).unwrap();
            env.set_object_array_element(jitems, idx as i32, val)
                .unwrap();
        }
        let items_obj = JObject::from_raw(jitems);
        make_optional(&env, items_obj.into()).unwrap().into_raw()
    }

    unsafe fn map_entries(
        self,
        env: jni::JNIEnv,
        obj_pointer: jobject,
        heads: Option<jobject>,
    ) -> jobject {
        let read = SomeRead::from_pointer(env, self);
        let obj = JavaObjId::from_raw(&env, obj_pointer).unwrap();
        let heads = heads.map(|h| heads_from_jobject(&env, h).unwrap());

        let entries = match read.object_type(&obj) {
            Ok(automerge::ObjType::Map) => match heads {
                Some(h) => read.map_range_at(obj, .., &h).collect::<Vec<_>>(),
                None => read.map_range(obj, ..).collect::<Vec<_>>(),
            },
            Ok(..) => return make_empty_option(&env).unwrap().into_raw(),
            Err(am::AutomergeError::NotAnObject) => {
                return make_empty_option(&env).unwrap().into_raw()
            }
            Err(e) => {
                env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
                return JObject::null().into_raw();
            }
        };
        let entries_arr = env
            .new_object_array(
                entries.len() as i32,
                am_classname!("MapEntry"),
                JObject::null(),
            )
            .unwrap();
        for (i, MapRangeItem { key, value, id, .. }) in entries.into_iter().enumerate() {
            let entry = env.alloc_object(am_classname!("MapEntry")).unwrap();
            env.set_field(
                entry,
                "key",
                "Ljava/lang/String;",
                env.new_string(key).unwrap().into(),
            )
            .unwrap();
            let am_val = to_amvalue(&env, (value, id)).unwrap();
            env.set_field(
                entry,
                "value",
                format!("L{};", am_classname!("AmValue")),
                am_val.into(),
            )
            .unwrap();
            env.set_object_array_element(entries_arr, i as i32, entry)
                .unwrap();
        }
        make_optional(&env, JObject::from_raw(entries_arr).into())
            .unwrap()
            .into_raw()
    }

    unsafe fn text(
        self,
        env: jni::JNIEnv,
        obj_pointer: jobject,
        heads: Option<jobject>,
    ) -> jobject {
        let read = SomeRead::from_pointer(env, self);
        let obj = JavaObjId::from_raw(&env, obj_pointer).unwrap();
        let heads = heads.map(|h| heads_from_jobject(&env, h).unwrap());
        let text = match read.object_type(&obj) {
            Ok(am::ObjType::Text) => match heads {
                Some(h) => read.text_at(obj, &h),
                None => read.text(obj),
            },
            Ok(..) => return make_empty_option(&env).unwrap().into_raw(),
            Err(am::AutomergeError::NotAnObject) => {
                return make_empty_option(&env).unwrap().into_raw()
            }
            Err(e) => {
                env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
                return JObject::null().into_raw();
            }
        };
        let text = catch!(env, text);
        let text = env.new_string(text).unwrap();
        make_optional(&env, text.into()).unwrap().into_raw()
    }

    unsafe fn marks(
        self,
        env: jni::JNIEnv,
        obj_pointer: jobject,
        heads_option: jobject,
    ) -> jobject {
        let read = SomeRead::from_pointer(env, self);
        let obj = JavaObjId::from_raw(&env, obj_pointer).unwrap();
        let heads_option = JObject::from_raw(heads_option);
        let heads_present = env
            .call_method(heads_option, "isPresent", "()Z", &[])
            .unwrap();
        let heads = if heads_present.z().unwrap() {
            let heads = env
                .call_method(heads_option, "get", "()Ljava/lang/Object;", &[])
                .unwrap()
                .l()
                .unwrap();
            Some(heads_from_jobject(&env, heads.into_raw()).unwrap())
        } else {
            None
        };
        let marks = if let Some(h) = heads {
            read.marks_at(obj, &h)
        } else {
            read.marks(obj)
        };
        let marks = match marks {
            Ok(m) => m,
            Err(e) => {
                env.throw_new(AUTOMERGE_EXCEPTION, e.to_string()).unwrap();
                return JObject::null().into_raw();
            }
        };
        let marks_arr = env.new_object("java/util/ArrayList", "()V", &[]).unwrap();
        for mark in marks {
            let jmark = mark_to_java(&env, &mark).unwrap();
            env.call_method(marks_arr, "add", "(Ljava/lang/Object;)Z", &[jmark.into()])
                .unwrap();
        }
        marks_arr.into_raw()
    }
}

// Existential type over all implementations of ReadOps
enum SomeRead<'a> {
    Transaction(&'a mut automerge::transaction::Transaction<'a>),
    Doc(&'a automerge::Automerge),
}

impl<'a> SomeRead<'a> {
    unsafe fn from_pointer(env: jni::JNIEnv<'a>, pointer: SomeReadPointer) -> SomeRead<'a> {
        match pointer {
            SomeReadPointer::Doc(doc_pointer) => Self::from_doc_pointer(env, doc_pointer),
            SomeReadPointer::Tx(tx) => Self::from_tx_pointer(env, tx),
        }
    }

    pub(crate) unsafe fn from_tx_pointer(env: jni::JNIEnv<'a>, pointer: jobject) -> SomeRead<'a> {
        let tx = AmTransaction::<'a>::from_pointer_obj(&env, pointer).unwrap();
        Self::Transaction(tx)
    }

    pub(crate) unsafe fn from_doc_pointer(env: jni::JNIEnv<'a>, pointer: jobject) -> SomeRead<'a> {
        let am = automerge::Automerge::from_pointer_obj(&env, pointer).unwrap();
        SomeRead::Doc(am)
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
    ) -> am::iter::MapRange<'b, R> {
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
    ) -> am::iter::MapRange<'b, R> {
        match self {
            SomeRead::Transaction(tx) => tx.map_range_at(obj, range, heads),
            SomeRead::Doc(doc) => doc.map_range_at(obj, range, heads),
        }
    }

    fn list_range<O: AsRef<am::ObjId>, R: std::ops::RangeBounds<usize>>(
        &self,
        obj: O,
        range: R,
    ) -> am::iter::ListRange<'_, R> {
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
    ) -> am::iter::ListRange<'_, R> {
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

    fn get_change_by_hash(&self, hash: &am::ChangeHash) -> Option<&am::Change> {
        match self {
            SomeRead::Transaction(tx) => tx.get_change_by_hash(hash),
            SomeRead::Doc(doc) => doc.get_change_by_hash(hash),
        }
    }

    fn marks<O: AsRef<am::ObjId>>(
        &self,
        obj: O,
    ) -> Result<Vec<am::marks::Mark<'_>>, am::AutomergeError> {
        match self {
            SomeRead::Transaction(tx) => tx.marks(obj),
            SomeRead::Doc(doc) => doc.marks(obj),
        }
    }

    fn marks_at<O: AsRef<am::ObjId>>(
        &self,
        obj: O,
        heads: &[am::ChangeHash],
    ) -> Result<Vec<am::marks::Mark<'_>>, am::AutomergeError> {
        match self {
            SomeRead::Transaction(tx) => tx.marks_at(obj, heads),
            SomeRead::Doc(doc) => doc.marks_at(obj, heads),
        }
    }

    fn get_cursor<O: AsRef<am::ObjId>>(
        &self,
        obj: O,
        position: usize,
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
}

impl<'a> ReadOps for SomeRead<'a> {
    fn heads(&self) -> Vec<am::ChangeHash> {
        match self {
            SomeRead::Transaction(tx) => tx.heads(),
            SomeRead::Doc(doc) => doc.heads(),
        }
    }
}
