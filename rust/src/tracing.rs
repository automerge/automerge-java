//! Native `initTracing` — bridges Rust `tracing` events to SLF4J loggers on
//! the Java side.
//!
//! The subscriber installs a custom [`Slf4jLayer`] that forwards every
//! `tracing` event to an SLF4J `Logger` obtained via
//! `org.slf4j.LoggerFactory.getLogger(target)`. Structured fields from the
//! `tracing` event are pushed into SLF4J's MDC (Mapped Diagnostic Context)
//! so that they appear in structured log output and can be consumed by
//! Logback, Log4J, etc.
//!
//! Repeat calls are no-ops: a global subscriber can only be installed
//! once per process, so subsequent invocations silently swallow the
//! would-be error. `LoadLibrary.initialize()` calls `initTracing` once,
//! but JNI class initialization can reorder across tests and we don't
//! want the second call to throw.
//!
//! # Filtering
//!
//! Level filtering is driven entirely by the SLF4J backend. The layer
//! implements [`Layer::register_callsite`] and [`Layer::enabled`] by
//! checking `logger.isXEnabled()` on the corresponding SLF4J logger,
//! and returns [`Interest::sometimes`] so that `tracing` re-evaluates
//! the filter when SLF4J levels change at runtime (e.g. via JMX or
//! Logback `<jmxConfigurator/>`).
//!
//! # Reentrancy
//!
//! Every JNI call the bridge makes (to look up a logger, check
//! `isXEnabled`, invoke a log method) travels through the `jni` crate,
//! which itself emits `tracing::trace!` events from its `jni::jvalue`
//! target. Without protection, those events would re-enter the bridge,
//! recurse without bound, and crash the JVM with a stack overflow.
//!
//! Two independent guards keep that from happening:
//!
//! 1. `register_callsite` returns [`Interest::never`] for any target
//!    beginning with `jni` (or its sub-targets like `jni::jvalue`), so
//!    the dispatcher skips those callsites entirely without asking.
//! 2. A thread-local [`IN_BRIDGE`] flag short-circuits
//!    [`Layer::enabled`] and [`Layer::on_event`] whenever we're already
//!    inside a bridge call — catch-all protection for any other
//!    target that tracing infrastructure might emit during our JNI
//!    traffic.

use jni::{
    objects::{JClass, JString},
    NativeMethod,
};
use std::cell::Cell;
use std::sync::Once;
use tracing::Level;
use tracing::Metadata;
use tracing_core::subscriber::Interest;
use tracing_subscriber::{layer::SubscriberExt, prelude::*};

use crate::bindings;

const _METHODS: &[NativeMethod] = &[
    ams_native! { static extern fn init_tracing() },
    ams_native! { static extern fn test_emit_tracing_event(level: JString, message: JString) },
];

static INIT: Once = Once::new();

thread_local! {
    /// Set while the current thread is executing inside the SLF4J
    /// bridge. Any `tracing` event emitted by that work (typically from
    /// the `jni` crate on each JNI call) short-circuits back out
    /// instead of recursing.
    static IN_BRIDGE: Cell<bool> = const { Cell::new(false) };
}

/// Run `f` with [`IN_BRIDGE`] set, guaranteeing it is cleared on the
/// way out even if `f` panics.
fn with_bridge_guard<R>(f: impl FnOnce() -> R) -> Option<R> {
    if IN_BRIDGE.with(|c| c.replace(true)) {
        // Already inside the bridge on this thread — drop the event.
        return None;
    }
    struct Reset;
    impl Drop for Reset {
        fn drop(&mut self) {
            IN_BRIDGE.with(|c| c.set(false));
        }
    }
    let _reset = Reset;
    Some(f())
}

fn init_tracing<'local>(
    env: &mut jni::Env<'local>,
    _class: JClass<'local>,
) -> jni::errors::Result<()> {
    // Obtain the JavaVM now, while we have an Env, so the layer can
    // attach from arbitrary Rust threads later.
    let jvm = env.get_java_vm()?;

    INIT.call_once(|| {
        let _ = tracing_subscriber::registry()
            .with(Slf4jLayer::new(jvm))
            .try_init();
    });
    Ok(())
}

/// Emit a `tracing` event at the given level with the given message.
///
/// This exists solely for smoke-testing the SLF4J bridge from Java-side
/// tests — it lets the test exercise `Slf4jLayer::on_event` without
/// depending on incidental tracing events inside automerge internals.
fn test_emit_tracing_event<'local>(
    _env: &mut jni::Env<'local>,
    _class: JClass<'local>,
    level: JString<'local>,
    message: JString<'local>,
) -> jni::errors::Result<()> {
    let level_str = level.to_string();
    let message_str = message.to_string();
    match level_str.as_str() {
        "trace" => tracing::trace!(target: "automerge::test", "{message_str}"),
        "debug" => tracing::debug!(target: "automerge::test", "{message_str}"),
        "info" => tracing::info!(target: "automerge::test", "{message_str}"),
        "warn" => tracing::warn!(target: "automerge::test", "{message_str}"),
        "error" => tracing::error!(target: "automerge::test", "{message_str}"),
        _ => tracing::info!(target: "automerge::test", "{message_str}"),
    }
    Ok(())
}

/// A [`tracing_subscriber::Layer`] that forwards `tracing` events to SLF4J
/// loggers on the Java side.
///
/// The logger name is the `tracing` target (e.g. `"automerge::sync"`), and
/// the event level is mapped to the corresponding SLF4J method (`trace`,
/// `debug`, `info`, `warn`, `error`). Structured fields other than `message`
/// are pushed into SLF4J's MDC for the duration of the log call so that
/// structured logging backends can pick them up.
///
/// Filtering is delegated to SLF4J via [`Layer::enabled`], which checks
/// `logger.isXEnabled()`. This means the SLF4J backend (Logback, Log4J,
/// etc.) is the single source of truth for which events are collected —
/// Rust-side `tracing` events that would be discarded by SLF4J are never
/// forwarded across the JNI boundary.
struct Slf4jLayer {
    jvm: jni::JavaVM,
}

impl Slf4jLayer {
    fn new(jvm: jni::JavaVM) -> Self {
        Self { jvm }
    }
}

impl<S: tracing::Subscriber> tracing_subscriber::Layer<S> for Slf4jLayer {
    /// Return [`Interest::sometimes`] so that `tracing` re-evaluates
    /// [`enabled`](Self::enabled) when SLF4J levels may have changed.
    fn register_callsite(&self, metadata: &'static Metadata<'static>) -> Interest {
        // The `jni` crate emits trace events (target `jni::jvalue`,
        // etc.) on every JNI call we'd make to evaluate this callsite.
        // Mark those as permanently disabled so the dispatcher doesn't
        // even ask — prevents unbounded recursion through the bridge.
        if is_jni_target(metadata.target()) {
            return Interest::never();
        }
        Interest::sometimes()
    }

    /// Ask the SLF4J logger whether the given level is enabled for this
    /// target. Events that SLF4J would discard are never collected by
    /// `tracing`, avoiding unnecessary JNI traffic.
    fn enabled(
        &self,
        metadata: &Metadata<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) -> bool {
        with_bridge_guard(|| slf4j_level_enabled(&self.jvm, metadata.target(), *metadata.level()))
            .unwrap_or(false)
    }

    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        with_bridge_guard(|| {
            let metadata = event.metadata();

            let mut visitor = FieldVisitor::default();
            event.record(&mut visitor);

            // Use the message field if present, otherwise fall back to the target.
            let message = visitor
                .message
                .unwrap_or_else(|| metadata.target().to_string());

            // Collect structured fields to push into MDC.
            let mut fields = visitor.fields;
            if let Some(file) = metadata.file() {
                fields.push(("file".to_string(), file.to_string()));
            }
            if let Some(line) = metadata.line() {
                fields.push(("line".to_string(), line.to_string()));
            }

            log_to_slf4j(
                &self.jvm,
                metadata.target(),
                *metadata.level(),
                &message,
                &fields,
            );
        });
    }
}

/// Whether `target` belongs to the `jni` crate (or a sub-target like
/// `jni::jvalue`). Those events describe the very JNI calls the bridge
/// makes, so forwarding them would recurse without bound.
fn is_jni_target(target: &str) -> bool {
    target == "jni" || target.starts_with("jni::")
}

/// Prefix applied to every SLF4J logger name so the whole library log
/// stream sits under one subtree that mirrors the Java package name.
const LOGGER_PREFIX: &str = "org.automerge";

/// Translate a Rust `tracing` target (e.g. `automerge::sync`) into an
/// SLF4J logger name that plays nicely with Logback/Log4J hierarchy
/// (e.g. `org.automerge.automerge.sync`).
///
/// Rust module paths use `::` as a separator, but SLF4J treats `.` as
/// the single hierarchical separator — so without this mapping, a
/// `<logger name="automerge">` entry would NOT cover `automerge::sync`,
/// and users would silently miss events.
fn target_to_logger_name(target: &str) -> String {
    let mut out = String::with_capacity(LOGGER_PREFIX.len() + 1 + target.len());
    out.push_str(LOGGER_PREFIX);
    out.push('.');
    out.push_str(&target.replace("::", "."));
    out
}

/// Check `logger.isXEnabled()` for the corresponding tracing level.
fn slf4j_level_enabled(jvm: &jni::JavaVM, target: &str, level: Level) -> bool {
    let logger_name = target_to_logger_name(target);
    let result = jvm.attach_current_thread(|env| -> jni::errors::Result<bool> {
        let j_target = env.new_string(&logger_name)?;
        let logger = bindings::Slf4jLoggerFactory::get_logger(env, &j_target)?;
        let enabled = match level {
            Level::TRACE => logger.is_trace_enabled(env)?,
            Level::DEBUG => logger.is_debug_enabled(env)?,
            Level::INFO => logger.is_info_enabled(env)?,
            Level::WARN => logger.is_warn_enabled(env)?,
            Level::ERROR => logger.is_error_enabled(env)?,
        };
        Ok(enabled)
    });
    result.unwrap_or(false)
}

#[derive(Default)]
struct FieldVisitor {
    message: Option<String>,
    fields: Vec<(String, String)>,
}

impl tracing::field::Visit for FieldVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        let value_str = format!("{:?}", value);
        if field.name() == "message" {
            self.message = Some(value_str);
        } else {
            self.fields.push((field.name().to_string(), value_str));
        }
    }

    fn record_str(&mut self, field: &tracing::field::Field, value: &str) {
        if field.name() == "message" {
            self.message = Some(value.to_string());
        } else {
            self.fields
                .push((field.name().to_string(), value.to_string()));
        }
    }
}

/// Call `LoggerFactory.getLogger(<mapped name>).<level>(message)`,
/// pushing `fields` into MDC for the duration of the call.
fn log_to_slf4j(
    jvm: &jni::JavaVM,
    target: &str,
    level: Level,
    message: &str,
    fields: &[(String, String)],
) {
    let logger_name = target_to_logger_name(target);
    let message = message.to_string();
    let fields = fields.to_vec();

    let result = jvm.attach_current_thread(|env| -> jni::errors::Result<()> {
        let j_logger_name = env.new_string(&logger_name)?;
        let logger = bindings::Slf4jLoggerFactory::get_logger(env, &j_logger_name)?;

        // --- Populate MDC with structured fields ---
        if !fields.is_empty() {
            for (key, value) in &fields {
                let j_key = env.new_string(key)?;
                let j_value = env.new_string(value)?;
                bindings::Slf4jMDC::put(env, &j_key, &j_value)?;
            }
        }

        // --- Call the SLF4J log method ---
        let j_message = env.new_string(&message)?;
        match level {
            Level::TRACE => logger.trace(env, &j_message)?,
            Level::DEBUG => logger.debug(env, &j_message)?,
            Level::INFO => logger.info(env, &j_message)?,
            Level::WARN => logger.warn(env, &j_message)?,
            Level::ERROR => logger.error(env, &j_message)?,
        }

        // --- Clear MDC ---
        if !fields.is_empty() {
            bindings::Slf4jMDC::clear(env)?;
        }

        Ok(())
    });

    if let Err(e) = result {
        eprintln!("automerge-jni: SLF4J logging bridge error: {e:?}");
    }
}
