//! Type bindings generated via `jni::bind_java_type!`.
//!
//! Each Java class we construct from Rust is declared here once, with its
//! constructors and fields. Callers then use the generated `Type<'local>`
//! wrappers and their typed accessors.

use jni::bind_java_type;

bind_java_type! {
    pub ObjectId => org.automerge.ObjectId,
    constructors {
        fn new(raw: jbyte[]),
    },
    fields { raw: jbyte[] },
}

bind_java_type! {
    pub JavaCounter => org.automerge.Counter,
    constructors {
        fn new(value: jlong),
    },
}

bind_java_type! {
    pub ChangeHash => org.automerge.ChangeHash,
    constructors { fn new(hash: jbyte[]) },
    fields { hash: jbyte[] },
}

bind_java_type! {
    pub Cursor => org.automerge.Cursor,
    constructors { fn new() },
    fields { raw: jbyte[] },
}

// Java enums: bind as (typed) static-field lookups plus an `ordinal()` method.
bind_java_type! {
    pub ObjectType => org.automerge.ObjectType,
    type_map = { ObjectType => org.automerge.ObjectType },
    methods {
        fn ordinal() -> jint,
    },
    fields {
        static map { sig = ObjectType, name = "MAP" },
        static list { sig = ObjectType, name = "LIST" },
        static text { sig = ObjectType, name = "TEXT" },
    },
}

bind_java_type! {
    pub ExpandMark => org.automerge.ExpandMark,
    type_map = { ExpandMark => org.automerge.ExpandMark },
    methods {
        fn ordinal() -> jint,
    },
    fields {
        static before { sig = ExpandMark, name = "BEFORE" },
        static after { sig = ExpandMark, name = "AFTER" },
        static both { sig = ExpandMark, name = "BOTH" },
        static none_variant { sig = ExpandMark, name = "NONE" },
    },
}

// java.util.Optional — static factory bindings.
bind_java_type! {
    pub Optional => java.util.Optional,
    methods {
        static fn of(value: JObject) -> Optional,
        static fn empty() -> Optional,
        fn is_present() -> jboolean,
        fn get() -> JObject,
    },
}

// java.util.ArrayList — just enough surface to build lists from Rust.
bind_java_type! {
    pub ArrayList => java.util.ArrayList,
    constructors { fn new() },
    methods {
        fn add(element: JObject) -> jboolean,
    },
}

// java.util.HashMap — just enough surface to build maps from Rust.
bind_java_type! {
    pub HashMap => java.util.HashMap,
    constructors { fn new() },
    methods {
        fn put(key: JObject, value: JObject) -> JObject,
    },
}

// AutomergeSys inner classes (the long-pointer holders). No constructors:
// Rust creates these via alloc_object + set_rust_field (see JavaPointer).
bind_java_type! { pub DocPointer => org.automerge.AutomergeSys::DocPointer }
bind_java_type! { pub TransactionPointer => org.automerge.AutomergeSys::TransactionPointer }
bind_java_type! { pub SyncStatePointer => org.automerge.AutomergeSys::SyncStatePointer }
bind_java_type! { pub PatchLogPointer => org.automerge.AutomergeSys::PatchLogPointer }

bind_java_type! {
    pub CommitResult => org.automerge.CommitResult,
    type_map = {
        Optional => java.util.Optional,
        PatchLogPointer => org.automerge.AutomergeSys::PatchLogPointer,
    },
    constructors {
        fn new(hash: Optional, patch_log: PatchLogPointer),
    },
}

// AmValue hierarchy ---------------------------------------------------------

bind_java_type! { pub AmValue => org.automerge.AmValue }

bind_java_type! {
    pub AmValueInt => org.automerge.AmValue::Int,
    type_map = { AmValue => org.automerge.AmValue },
    is_instance_of = { base: AmValue },
    constructors { fn new() },
    fields { value: jlong },
}

bind_java_type! {
    pub AmValueUInt => org.automerge.AmValue::UInt,
    type_map = { AmValue => org.automerge.AmValue },
    is_instance_of = { base: AmValue },
    constructors { fn new() },
    fields { value: jlong },
}

bind_java_type! {
    pub AmValueBool => org.automerge.AmValue::Bool,
    type_map = { AmValue => org.automerge.AmValue },
    is_instance_of = { base: AmValue },
    constructors { fn new() },
    fields { value: jboolean },
}

bind_java_type! {
    pub AmValueBytes => org.automerge.AmValue::Bytes,
    type_map = { AmValue => org.automerge.AmValue },
    is_instance_of = { base: AmValue },
    constructors { fn new() },
    fields { value: jbyte[] },
}

bind_java_type! {
    pub AmValueStr => org.automerge.AmValue::Str,
    type_map = { AmValue => org.automerge.AmValue },
    is_instance_of = { base: AmValue },
    constructors { fn new() },
    fields { value: JString },
}

bind_java_type! {
    pub AmValueF64 => org.automerge.AmValue::F64,
    type_map = { AmValue => org.automerge.AmValue },
    is_instance_of = { base: AmValue },
    constructors { fn new() },
    fields { value: jdouble },
}

bind_java_type! {
    pub AmValueCounter => org.automerge.AmValue::Counter,
    type_map = {
        AmValue => org.automerge.AmValue,
        JavaCounter => org.automerge.Counter,
    },
    is_instance_of = { base: AmValue },
    constructors { fn new() },
    fields { value: JavaCounter },
}

bind_java_type! {
    pub AmValueTimestamp => org.automerge.AmValue::Timestamp,
    type_map = { AmValue => org.automerge.AmValue },
    is_instance_of = { base: AmValue },
    constructors { fn new() },
    fields { value: java.util.Date },
}

bind_java_type! {
    pub AmValueNull => org.automerge.AmValue::Null,
    type_map = { AmValue => org.automerge.AmValue },
    is_instance_of = { base: AmValue },
    constructors { fn new() },
}

bind_java_type! {
    pub AmValueUnknown => org.automerge.AmValue::Unknown,
    type_map = { AmValue => org.automerge.AmValue },
    is_instance_of = { base: AmValue },
    constructors { fn new() },
    fields {
        type_code {
            sig = jint,
            name = "typeCode",
        },
        value: jbyte[],
    },
}

bind_java_type! {
    pub AmValueMap => org.automerge.AmValue::Map,
    type_map = {
        AmValue => org.automerge.AmValue,
        ObjectId => org.automerge.ObjectId,
    },
    is_instance_of = { base: AmValue },
    constructors { fn new() },
    fields { id: ObjectId },
}

bind_java_type! {
    pub AmValueList => org.automerge.AmValue::List,
    type_map = {
        AmValue => org.automerge.AmValue,
        ObjectId => org.automerge.ObjectId,
    },
    is_instance_of = { base: AmValue },
    constructors { fn new() },
    fields { id: ObjectId },
}

bind_java_type! {
    pub AmValueText => org.automerge.AmValue::Text,
    type_map = {
        AmValue => org.automerge.AmValue,
        ObjectId => org.automerge.ObjectId,
    },
    is_instance_of = { base: AmValue },
    constructors { fn new() },
    fields { id: ObjectId },
}

bind_java_type! {
    pub MapEntry => org.automerge.MapEntry,
    type_map = { AmValue => org.automerge.AmValue },
    constructors { fn new() },
    fields {
        key: JString,
        value: AmValue,
    },
}

bind_java_type! {
    pub Conflicts => org.automerge.Conflicts,
    type_map = { HashMap => java.util.HashMap },
    constructors { fn new() },
    fields { values: HashMap },
}

// NewValue hierarchy -------------------------------------------------------

bind_java_type! { pub NewValue => org.automerge.NewValue }

bind_java_type! {
    pub NewValueInt => org.automerge.NewValue::Int,
    type_map = { NewValue => org.automerge.NewValue },
    is_instance_of = { base: NewValue },
    fields { value: jlong },
}

bind_java_type! {
    pub NewValueUInt => org.automerge.NewValue::UInt,
    type_map = { NewValue => org.automerge.NewValue },
    is_instance_of = { base: NewValue },
    fields { value: jlong },
}

bind_java_type! {
    pub NewValueF64 => org.automerge.NewValue::F64,
    type_map = { NewValue => org.automerge.NewValue },
    is_instance_of = { base: NewValue },
    fields { value: jdouble },
}

bind_java_type! {
    pub NewValueBool => org.automerge.NewValue::Bool,
    type_map = { NewValue => org.automerge.NewValue },
    is_instance_of = { base: NewValue },
    fields { value: jboolean },
}

bind_java_type! {
    pub NewValueStr => org.automerge.NewValue::Str,
    type_map = { NewValue => org.automerge.NewValue },
    is_instance_of = { base: NewValue },
    fields { value: JString },
}

bind_java_type! {
    pub NewValueBytes => org.automerge.NewValue::Bytes,
    type_map = { NewValue => org.automerge.NewValue },
    is_instance_of = { base: NewValue },
    fields { value: jbyte[] },
}

bind_java_type! {
    pub NewValueCounter => org.automerge.NewValue::Counter,
    type_map = { NewValue => org.automerge.NewValue },
    is_instance_of = { base: NewValue },
    fields { value: jlong },
}

bind_java_type! {
    pub NewValueTimestamp => org.automerge.NewValue::Timestamp,
    type_map = { NewValue => org.automerge.NewValue },
    is_instance_of = { base: NewValue },
    fields { value: java.util.Date },
}

bind_java_type! {
    pub NewValueNull => org.automerge.NewValue::Null,
    type_map = { NewValue => org.automerge.NewValue },
    is_instance_of = { base: NewValue },
}

// Prop hierarchy -----------------------------------------------------------

bind_java_type! { pub Prop => org.automerge.Prop }

bind_java_type! {
    pub PropKey => org.automerge.Prop::Key,
    type_map = { Prop => org.automerge.Prop },
    is_instance_of = { base: Prop },
    constructors { fn new(key: JString) },
}

bind_java_type! {
    pub PropIndex => org.automerge.Prop::Index,
    type_map = { Prop => org.automerge.Prop },
    is_instance_of = { base: Prop },
    constructors { fn new(index: jlong) },
}

// PathElement --------------------------------------------------------------

bind_java_type! {
    pub PathElement => org.automerge.PathElement,
    type_map = {
        ObjectId => org.automerge.ObjectId,
        Prop => org.automerge.Prop,
    },
    constructors { fn new(object_id: ObjectId, prop: Prop) },
}

// Mark ---------------------------------------------------------------------

bind_java_type! {
    pub Mark => org.automerge.Mark,
    type_map = { AmValue => org.automerge.AmValue },
    constructors { fn new(start: jlong, end: jlong, name: JString, value: AmValue) },
}

// Patch + PatchAction hierarchy -------------------------------------------

bind_java_type! {
    pub Patch => org.automerge.Patch,
    type_map = {
        ObjectId => org.automerge.ObjectId,
        PathElement => org.automerge.PathElement,
        PatchAction => org.automerge.PatchAction,
    },
    constructors {
        fn new(obj: ObjectId, path: PathElement[], action: PatchAction),
    },
}

bind_java_type! { pub PatchAction => org.automerge.PatchAction }

bind_java_type! {
    pub PutMap => org.automerge.PatchAction::PutMap,
    type_map = {
        PatchAction => org.automerge.PatchAction,
        AmValue => org.automerge.AmValue,
    },
    is_instance_of = { base: PatchAction },
    constructors { fn new(key: JString, value: AmValue, conflict: jboolean) },
}

bind_java_type! {
    pub PutList => org.automerge.PatchAction::PutList,
    type_map = {
        PatchAction => org.automerge.PatchAction,
        AmValue => org.automerge.AmValue,
    },
    is_instance_of = { base: PatchAction },
    constructors { fn new(index: jlong, value: AmValue, conflict: jboolean) },
}

bind_java_type! {
    pub PatchActionInsert => org.automerge.PatchAction::Insert,
    type_map = {
        PatchAction => org.automerge.PatchAction,
        AmValue => org.automerge.AmValue,
    },
    is_instance_of = { base: PatchAction },
    constructors { fn new(index: jlong, values: AmValue[]) },
}

bind_java_type! {
    pub SpliceText => org.automerge.PatchAction::SpliceText,
    type_map = { PatchAction => org.automerge.PatchAction },
    is_instance_of = { base: PatchAction },
    constructors { fn new(index: jlong, text: JString) },
}

bind_java_type! {
    pub Increment => org.automerge.PatchAction::Increment,
    type_map = {
        PatchAction => org.automerge.PatchAction,
        Prop => org.automerge.Prop,
    },
    is_instance_of = { base: PatchAction },
    constructors { fn new(property: Prop, value: jlong) },
}

bind_java_type! {
    pub DeleteMap => org.automerge.PatchAction::DeleteMap,
    type_map = { PatchAction => org.automerge.PatchAction },
    is_instance_of = { base: PatchAction },
    constructors { fn new(key: JString) },
}

bind_java_type! {
    pub DeleteList => org.automerge.PatchAction::DeleteList,
    type_map = { PatchAction => org.automerge.PatchAction },
    is_instance_of = { base: PatchAction },
    constructors { fn new(index: jlong, length: jlong) },
}

bind_java_type! {
    pub PatchActionMark => org.automerge.PatchAction::Mark,
    type_map = {
        PatchAction => org.automerge.PatchAction,
        Mark => org.automerge.Mark,
    },
    is_instance_of = { base: PatchAction },
    constructors { fn new(marks: Mark[]) },
}

bind_java_type! {
    pub FlagConflict => org.automerge.PatchAction::FlagConflict,
    type_map = {
        PatchAction => org.automerge.PatchAction,
        Prop => org.automerge.Prop,
    },
    is_instance_of = { base: PatchAction },
    constructors { fn new(property: Prop) },
}
