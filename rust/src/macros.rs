#![macro_use]

// Prefix a JNI type name with the automerge package path
macro_rules! am_classname {
    ($name:literal) => {
        ::jni::jni_str!("org/automerge/", $name)
    };
}

/// Wrapper around `jni::native_method!` that injects the `java_type` and the
/// `type_map` common to every native method on `org.automerge.AutomergeSys`,
/// so call sites only have to write the method signature itself:
///
/// ```ignore
/// ams_native! { static extern fn create_doc() -> bindings::DocPointer }
/// ```
///
/// # Background
///
/// `jni::bind_java_type!` (used throughout `bindings.rs`) generates a
/// strongly-typed Rust wrapper — e.g. `bindings::DocPointer<'local>` — for
/// each Java class we construct from Rust. Under the hood the wrapper is a
/// transparent `jobject` newtype with cached `jclass` / `jmethodID` /
/// `jfieldID` lookups so calls are direct JNI dispatch, not string-based
/// resolution.
///
/// `jni::native_method!` is the sibling macro for declaring a Rust
/// implementation of a Java `native` method. It reads the signature,
/// derives the JNI mangled form (e.g.
/// `(Lorg/automerge/AutomergeSys$DocPointer;)V`), and emits the
/// `extern "system"` trampoline the JVM expects. To do that it has to know
/// which Rust type stands in for which Java class — that's the `type_map`:
///
/// ```ignore
/// type_map = {
///     bindings::DocPointer => org.automerge.AutomergeSys::DocPointer,
///     // …one entry per bound type that might appear in any signature…
/// }
/// ```
///
/// Each native method on `AutomergeSys` uses the same `java_type` and the
/// same (full) `type_map`, so rather than repeat them 100+ times we hoist
/// both into this macro.
///
/// # Why paths, not bare idents
///
/// The `type_map` entries reference the Rust wrapper types by **path**
/// (`bindings::DocPointer`, not `DocPointer`). That means a caller only
/// needs `use crate::bindings;` in scope — a single stable import — rather
/// than naming every bound type individually. Adding a new wrapper to
/// `bindings.rs` and to this macro's `type_map` is then the only edit
/// needed; no per-file import lists have to be kept in sync.
///
/// **To add a new bound type, append one line to the `type_map` below.**
macro_rules! ams_native {
    ($($tt:tt)*) => {
        ::jni::native_method! {
            java_type = org.automerge.AutomergeSys,
            type_map = {
                bindings::DocPointer         => org.automerge.AutomergeSys::DocPointer,
                bindings::TransactionPointer => org.automerge.AutomergeSys::TransactionPointer,
                bindings::SyncStatePointer   => org.automerge.AutomergeSys::SyncStatePointer,
                bindings::PatchLogPointer    => org.automerge.AutomergeSys::PatchLogPointer,
                bindings::ChangeHash         => org.automerge.ChangeHash,
                bindings::ObjectId           => org.automerge.ObjectId,
                bindings::AmValue            => org.automerge.AmValue,
                bindings::Patch              => org.automerge.Patch,
                bindings::CommitResult       => org.automerge.CommitResult,
                bindings::Optional           => java.util.Optional,
                bindings::ArrayList          => java.util.ArrayList,
                bindings::HashMap            => java.util.HashMap,
                bindings::Cursor             => org.automerge.Cursor,
                bindings::Mark               => org.automerge.Mark,
                bindings::MapEntry           => org.automerge.MapEntry,
                bindings::ExpandMark        => org.automerge.ExpandMark,
            },
            $($tt)*
        }
    };
}
