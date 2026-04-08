//! CLI Tracing
//!
//! The CLI's tracing has internal and user-facing logs. User-facing logs are directly routed to the user in some form.
//! Internal logs are stored in a log file for consumption in bug reports and debugging.
//! We use tracing fields to determine whether a log is internal or external and additionally if the log should be
//! formatted or not.
//!
//! These two fields are
//! `dx_src` which tells the logger that this is a user-facing message and should be routed as so.
//! `dx_no_fmt`which tells the logger to avoid formatting the log and to print it as-is.
//!
//! 1. Build general filter
//! 2. Build file append layer for logging to a file. This file is reset on every CLI-run.
//! 3. Build CLI layer for routing tracing logs to the TUI.
//! 4. Build fmt layer for non-interactive logging with a custom writer that prevents output during interactive mode.

use std::{
    any::Any,
    borrow::Cow,
    collections::HashMap,
    env,
    fmt::{
        Debug,
        Display,
        Write as _,
    },
    future::Future,
    panic::AssertUnwindSafe,
    pin::Pin,
    str::FromStr,
    sync::{
        atomic::{
            AtomicBool,
            Ordering,
        },
        Arc,
        OnceLock,
    },
    time::{
        Duration,
        Instant,
    },
};

use anyhow::{
    Context,
    Error,
    Result,
};
use cargo_metadata::diagnostic::{
    Diagnostic,
    DiagnosticLevel,
};
use clap::Parser;
use futures_channel::mpsc::{
    UnboundedReceiver,
    UnboundedSender,
};
use futures_util::FutureExt;
use itertools::Itertools;
use tracing::{
    field::Visit,
    Level,
    Subscriber,
};
use tracing_subscriber::{
    fmt::{
        format::{
            self,
            Writer,
        },
        time::FormatTime,
    },
    prelude::*,
    registry::LookupSpan,
    EnvFilter,
    Layer,
};

use crate::{
    serve::ServeUpdate,
    BundleFormat,
    Cli,
    Commands,
    StructuredOutput,
    Verbosity,
};

const LOG_ENV: &str = "FREYA_LOG";
const DX_SRC_FLAG: &str = "dx_src";

pub static VERBOSITY: OnceLock<Verbosity> = OnceLock::new();

pub fn verbosity_or_default() -> Verbosity {
    crate::VERBOSITY.get().cloned().unwrap_or_default()
}

fn reset_cursor() {
    use std::io::IsTerminal;

    if std::io::stdout().is_terminal() {
        _ = console::Term::stdout().show_cursor();
    }
}

/// A trait that emits an anonymous JSON representation of the object, suitable for telemetry.
pub(crate) trait Anonymized {
    fn anonymized(&self) -> serde_json::Value;
}

/// A custom layer that wraps our special interception logic based on the mode of the CLI and its verbosity.
///
/// Redirects TUI logs and writes to files.
#[derive(Clone)]
pub struct TraceController {
    log_to_file: Option<Arc<tokio::sync::Mutex<std::fs::File>>>,
    tui_active: Arc<AtomicBool>,
    tui_tx: UnboundedSender<TraceMsg>,
    tui_rx: Arc<tokio::sync::Mutex<UnboundedReceiver<TraceMsg>>>,
}

/// An error that contains information about a captured panic.
#[derive(Debug, Clone)]
struct CapturedPanicError {
    error: Arc<Error>,
    error_type: String,
    thread_name: Option<String>,
    location: Option<SavedLocation>,
}
impl std::fmt::Display for CapturedPanicError {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(
            f,
            "CapturedBacktrace {{ error: {}, error_type: {}, thread_name: {:?}, location: {:?} }}",
            self.error, self.error_type, self.thread_name, self.location
        )
    }
}
impl std::error::Error for CapturedPanicError {}

#[allow(unused)]
#[derive(Debug, Clone)]
struct SavedLocation {
    file: String,
    line: u32,
    column: u32,
}

impl TraceController {
    /// Initialize the CLI and set up the tracing infrastructure.
    ///
    /// This captures panics and handles logging setup.
    pub async fn main(
        run_app: impl FnOnce(Commands, Self) -> Pin<Box<dyn Future<Output = Result<StructuredOutput>>>>,
    ) -> StructuredOutput {
        let args = Cli::parse();
        let tui_active = Arc::new(AtomicBool::new(false));
        let is_serve_cmd = matches!(args.action, Commands::Serve(_));

        VERBOSITY
            .set(args.verbosity.clone())
            .expect("verbosity should only be set once");

        // Set up a basic env-based filter for the logs
        let cli_target = env!("CARGO_PKG_NAME").replace('-', "_");

        let env_filter = match env::var(LOG_ENV) {
            Ok(_) => EnvFilter::from_env(LOG_ENV),
            _ if is_serve_cmd => {
                EnvFilter::new(format!(
                    "error,{cli_target}=trace,freya_core=trace,freya_hotreload=trace,subsecond_cli_support=trace"
                ))
            }
            _ => EnvFilter::new(format!(
                "error,{cli_target}={our_level},freya_core={our_level},freya_hotreload={our_level},subsecond_cli_support={our_level}",
                our_level = if args.verbosity.verbose {
                    "debug"
                } else {
                    "info"
                }
            )),
        };

        // Listen to a few more tokio events if the tokio-console feature is enabled
        #[cfg(feature = "tokio-console")]
        let env_filter = env_filter
            .add_directive("tokio=trace".parse().unwrap())
            .add_directive("runtime=trace".parse().unwrap());

        // Set up the json filter which lets through JSON traces only if the `json` field is present
        let json_filter = tracing_subscriber::filter::filter_fn(move |meta| {
            if meta.fields().len() == 1 && meta.fields().iter().next().unwrap().name() == "json" {
                return args.verbosity.json_output;
            }
            true
        });

        // We filter out a few fields that are not relevant to the user
        let fmt_layer = tracing_subscriber::fmt::layer()
            .with_target(false)
            .fmt_fields(
                format::debug_fn(move |writer, field, value| {
                    if field.name() == "json" && !args.verbosity.json_output {
                        return Ok(());
                    }

                    if field.name() == "dx_src" && !args.verbosity.verbose {
                        return Ok(());
                    }

                    write!(writer, "{}", format_field(field.name(), value))
                })
                .delimited(" "),
            )
            .with_timer(PrettyUptime::default());

        let fmt_layer = if args.verbosity.json_output {
            fmt_layer.json().flatten_event(true).boxed()
        } else {
            fmt_layer.boxed()
        }
        .with_filter(tracing_subscriber::filter::filter_fn({
            let tui_active = tui_active.clone();
            move |re| {
                if tui_active.load(Ordering::Relaxed) {
                    return false;
                }

                if !is_serve_cmd {
                    return true;
                }

                let verbosity = VERBOSITY.get().unwrap();

                if verbosity.trace {
                    return re.level() <= &Level::TRACE;
                }

                if verbosity.verbose {
                    return re.level() <= &Level::DEBUG;
                }

                re.level() <= &Level::INFO
            }
        }));

        #[cfg(feature = "tokio-console")]
        let console_layer = console_subscriber::spawn();
        #[cfg(not(feature = "tokio-console"))]
        let console_layer = tracing_subscriber::layer::Identity::new();

        let log_to_file = args
            .verbosity
            .log_to_file
            .as_deref()
            .map(|file_path| {
                std::fs::OpenOptions::new()
                    .append(true)
                    .create(true)
                    .open(file_path)
                    .map(|file| Arc::new(tokio::sync::Mutex::new(file)))
            })
            .transpose()
            .context("Failed to open specified log_file for writing")
            .unwrap();

        let (tui_tx, tui_rx) = futures_channel::mpsc::unbounded();
        let tracer = TraceController {
            log_to_file,
            tui_tx,
            tui_rx: Arc::new(tokio::sync::Mutex::new(tui_rx)),
            tui_active,
        };

        // Construct the tracing subscriber
        tracing_subscriber::registry()
            .with(env_filter)
            .with(json_filter)
            .with(tracer.clone())
            .with(console_layer)
            .with(fmt_layer)
            .init();

        // Set the panic handler to capture backtraces in case of a panic
        std::panic::set_hook(Box::new(move |panic_info| {
            let payload = if let Some(s) = panic_info.payload().downcast_ref::<String>() {
                s.to_string()
            } else if let Some(s) = panic_info.payload().downcast_ref::<&str>() {
                s.to_string()
            } else {
                "<unknown panic>".to_string()
            };

            let current_thread = std::thread::current();
            let thread_name = current_thread.name().map(|s| s.to_string());
            let location = panic_info.location().unwrap();
            let err = anyhow::anyhow!(payload);
            let err_display = format!("{:?}", err)
                .lines()
                .take_while(|line| !line.ends_with("___rust_try"))
                .join("\n");

            let boxed_panic: Box<dyn std::error::Error + Send + 'static> =
                Box::new(CapturedPanicError {
                    error: Arc::new(err),
                    thread_name: thread_name.clone(),
                    error_type: "Rust Panic".to_string(),
                    location: panic_info.location().map(|l| SavedLocation {
                        file: l.file().to_string(),
                        line: l.line(),
                        column: l.column(),
                    }),
                });

            tracing::error!(
                backtrace = boxed_panic,
                "Thread {} panicked at {location}:\n\n               {err_display}",
                thread_name.as_deref().unwrap_or("<unknown>"),
            );
        }));

        // Run the app, catching panics and errors
        let app_res = AssertUnwindSafe(run_with_ctrl_c(run_app(args.action, tracer.clone())))
            .catch_unwind()
            .await;

        tracer.finish(app_res).await
    }

    // Redirects the tracing logs to the TUI if it's active, otherwise it just collects them.
    pub fn redirect_to_tui(&self) {
        self.tui_active.store(true, Ordering::Relaxed);
    }

    /// Wait for the internal logger to send a message
    pub(crate) async fn wait(&self) -> ServeUpdate {
        use futures_util::StreamExt;

        let Some(log) = self.tui_rx.lock().await.next().await else {
            return std::future::pending().await;
        };

        ServeUpdate::TracingLog { log }
    }

    pub(crate) async fn drain_pending(&self) -> Vec<TraceMsg> {
        let mut logs = Vec::new();
        let mut rx = self.tui_rx.lock().await;

        while let Ok(log) = rx.try_recv() {
            logs.push(log);
        }

        logs
    }

    async fn finish(
        &self,
        res: Result<Result<StructuredOutput>, Box<dyn Any + Send>>,
    ) -> StructuredOutput {
        self.tui_active.store(false, Ordering::Relaxed);
        reset_cursor();

        // re-emit any remaining messages in case they're useful.
        while let Ok(msg) = self.tui_rx.lock().await.try_recv() {
            let content = match msg.content {
                TraceContent::Text(text) => text,
                TraceContent::Cargo(msg) => msg.message.to_string(),
            };
            match msg.level {
                Level::ERROR => tracing::error!("{content}"),
                Level::WARN => tracing::warn!("{content}"),
                Level::INFO => tracing::info!("{content}"),
                Level::DEBUG => tracing::debug!("{content}"),
                Level::TRACE => tracing::trace!("{content}"),
            }
        }

        match res {
            Ok(Ok(output)) => output,

            Ok(Err(err)) => {
                use crate::styles::{
                    ERROR_STYLE,
                    GLOW_STYLE,
                };
                let arg = std::env::args().nth(1).unwrap_or_else(|| "dx".to_string());
                let err_display = format!("{err:?}")
                    .lines()
                    .take_while(|line| !line.ends_with("___rust_try"))
                    .join("\n");
                let message = format!(
                    "{ERROR_STYLE}ERROR{ERROR_STYLE:#} {GLOW_STYLE}dx {}{GLOW_STYLE:#}: {}",
                    arg, err_display
                );
                eprintln!("\n{message}");
                StructuredOutput::Error { message }
            }

            Err(e) => StructuredOutput::Error {
                message: if let Some(s) = e.downcast_ref::<String>() {
                    s.to_string()
                } else if let Some(s) = e.downcast_ref::<&str>() {
                    s.to_string()
                } else {
                    "<unknown error>".to_string()
                },
            },
        }
    }
}

impl<S> Layer<S> for TraceController
where
    S: Subscriber + for<'a> LookupSpan<'a>,
{
    fn on_event(
        &self,
        event: &tracing::Event<'_>,
        _ctx: tracing_subscriber::layer::Context<'_, S>,
    ) {
        let mut visitor = CollectVisitor::default();
        event.record(&mut visitor);
        let level = event.metadata().level();

        // Redirect to the TUI if it's active
        if self.tui_active.load(Ordering::Relaxed) {
            let final_msg = visitor.pretty();

            if visitor.source == TraceSrc::Unknown {
                visitor.source = TraceSrc::Dev;
            }

            _ = self
                .tui_tx
                .unbounded_send(TraceMsg::text(visitor.source, *level, final_msg));
        }

        // Write to a file if we need to.
        if let Some(open_file) = self.log_to_file.as_ref() {
            let new_line = if visitor.source == TraceSrc::Cargo {
                Cow::Borrowed(visitor.message.as_str())
            } else {
                let meta = event.metadata();
                let level = meta.level();

                let mut final_msg = String::new();
                _ = write!(
                    final_msg,
                    "[{level}] {}: {} ",
                    meta.module_path().unwrap_or("dx"),
                    visitor.message.as_str()
                );

                for (field, value) in visitor.fields.iter() {
                    _ = write!(final_msg, "{} ", format_field(field, value));
                }
                _ = writeln!(final_msg);
                Cow::Owned(final_msg)
            };

            let new_data = console::strip_ansi_codes(&new_line).to_string();
            if let Ok(mut file) = open_file.try_lock() {
                use std::io::Write;
                _ = writeln!(file, "{}", new_data);
            }
        }
    }
}

/// A record visitor that collects dx-specific info and user-provided fields for logging consumption.
#[derive(Default)]
struct CollectVisitor {
    message: String,
    source: TraceSrc,
    fields: HashMap<String, String>,
    captured_panic: Option<CapturedPanicError>,
}

impl Visit for CollectVisitor {
    fn record_debug(&mut self, field: &tracing::field::Field, value: &dyn std::fmt::Debug) {
        let name = field.name();

        let mut value_string = String::new();
        write!(value_string, "{value:?}").unwrap();

        if name == "message" {
            self.message = value_string;
            return;
        }

        if name == DX_SRC_FLAG {
            self.source = TraceSrc::from(value_string);
            return;
        }

        if name == "json" || name == "backtrace" {
            return;
        }

        self.fields.insert(name.to_string(), value_string);
    }

    fn record_error(
        &mut self,
        field: &tracing::field::Field,
        value: &(dyn std::error::Error + 'static),
    ) {
        if let Some(captured) = value.downcast_ref::<CapturedPanicError>() {
            self.captured_panic = Some(captured.clone());
        } else {
            self.record_debug(field, &tracing::field::display(value));
        }
    }
}

impl CollectVisitor {
    fn pretty(&self) -> String {
        let mut final_msg = String::new();
        write!(final_msg, "{} ", self.message).unwrap();

        for (field, value) in self.fields.iter() {
            if field == "json" || field == "backtrace" {
                continue;
            }

            write!(final_msg, "{} ", format_field(field, value)).unwrap();
        }
        final_msg
    }
}

/// Formats a tracing field and value, removing any internal fields from the final output.
fn format_field(field_name: &str, value: &dyn Debug) -> String {
    match field_name {
        "message" => format!("{value:?}"),
        _ => format!("{field_name}={value:?}"),
    }
}

#[derive(Clone, PartialEq)]
pub struct TraceMsg {
    pub source: TraceSrc,
    pub level: Level,
    pub content: TraceContent,
    pub timestamp: std::time::SystemTime,
}

#[derive(Clone, PartialEq)]
#[allow(clippy::large_enum_variant)]
pub enum TraceContent {
    Cargo(Diagnostic),
    Text(String),
}

impl TraceMsg {
    pub fn text(source: TraceSrc, level: Level, content: String) -> Self {
        Self {
            source,
            level,
            content: TraceContent::Text(content),
            timestamp: std::time::SystemTime::now(),
        }
    }

    /// Create a new trace message from a cargo compiler message
    pub fn cargo(content: Diagnostic) -> Self {
        Self {
            level: match content.level {
                DiagnosticLevel::Ice => Level::ERROR,
                DiagnosticLevel::Error => Level::ERROR,
                DiagnosticLevel::FailureNote => Level::ERROR,
                DiagnosticLevel::Warning => Level::TRACE,
                DiagnosticLevel::Note => Level::TRACE,
                DiagnosticLevel::Help => Level::TRACE,
                _ => Level::TRACE,
            },
            timestamp: std::time::SystemTime::now(),
            source: TraceSrc::Cargo,
            content: TraceContent::Cargo(content),
        }
    }
}

#[derive(Clone, Copy, PartialEq, Default)]
pub enum TraceSrc {
    App(BundleFormat),
    Dev,
    Build,
    Cargo,

    #[default]
    Unknown,
}

impl std::fmt::Debug for TraceSrc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        <Self as Display>::fmt(self, f)
    }
}

impl From<String> for TraceSrc {
    fn from(value: String) -> Self {
        match value.as_str() {
            "dev" => Self::Dev,
            "bld" => Self::Build,
            "cargo" => Self::Cargo,
            other => BundleFormat::from_str(other)
                .map(Self::App)
                .unwrap_or_else(|_| Self::Unknown),
        }
    }
}

impl Display for TraceSrc {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Self::App(bundle) => write!(f, "{bundle}"),
            Self::Dev => write!(f, "dev"),
            Self::Build => write!(f, "build"),
            Self::Cargo => write!(f, "cargo"),
            Self::Unknown => write!(f, "n/a"),
        }
    }
}

/// Retrieve and print the relative elapsed wall-clock time since an epoch.
#[derive(Debug, Clone, Copy, Eq, PartialEq)]
pub struct PrettyUptime {
    epoch: Instant,
}

impl Default for PrettyUptime {
    fn default() -> Self {
        Self {
            epoch: Instant::now(),
        }
    }
}

impl FormatTime for PrettyUptime {
    fn format_time(&self, w: &mut Writer<'_>) -> std::fmt::Result {
        Self::write_elapsed(self.epoch.elapsed(), w)
    }
}

impl PrettyUptime {
    fn write_elapsed(elapsed: Duration, mut w: impl std::fmt::Write) -> std::fmt::Result {
        write!(
            w,
            "{:3}.{:02}s",
            elapsed.as_secs(),
            elapsed.subsec_millis() / 10
        )
    }
}

async fn run_with_ctrl_c<F: Future>(fut: F) -> F::Output {
    let ctrl_c = tokio::signal::ctrl_c();
    tokio::select! {
        res = fut => res,
        _ = ctrl_c => {
            reset_cursor();
            std::process::exit(0);
        }
    }
}

#[cfg(test)]
mod pretty_uptime_tests {
    use std::time::Duration;

    use super::PrettyUptime;

    #[test]
    fn pretty_uptime_pads_centiseconds_to_keep_a_stable_width() {
        let cases = [
            (Duration::from_millis(92_993), " 92.99s"),
            (Duration::from_millis(93_085), " 93.08s"),
            (Duration::from_millis(93_130), " 93.13s"),
            (Duration::from_millis(999_999), "999.99s"),
        ];

        for (elapsed, expected) in cases {
            let mut rendered = String::new();
            PrettyUptime::write_elapsed(elapsed, &mut rendered).unwrap();
            assert_eq!(rendered, expected);
        }
    }
}
