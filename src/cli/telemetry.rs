use anyhow::Result;
use std::sync::OnceLock;
use tracing::Level;
use tracing_subscriber::{EnvFilter, Registry, fmt, layer::SubscriberExt};

static TELEMETRY_INIT: OnceLock<()> = OnceLock::new();

/// Initialize tracing for the CLI process.
///
/// # Errors
///
/// Returns an error if the tracing filter cannot be parsed or the global
/// subscriber cannot be installed.
pub fn init(verbosity_level: Option<Level>) -> Result<()> {
    if TELEMETRY_INIT.get().is_some() {
        return Ok(());
    }

    let default_level = verbosity_level.unwrap_or(Level::ERROR);
    let fmt_layer = fmt::layer()
        .with_file(false)
        .with_line_number(false)
        .with_target(false)
        .with_thread_ids(false)
        .with_thread_names(false)
        .compact();

    let filter = EnvFilter::builder()
        .with_default_directive(default_level.into())
        .from_env_lossy()
        .add_directive("hyper=error".parse()?)
        .add_directive("tokio=error".parse()?);

    let subscriber = Registry::default().with(filter).with(fmt_layer);
    tracing::subscriber::set_global_default(subscriber)?;
    let _ = TELEMETRY_INIT.set(());
    Ok(())
}

pub fn shutdown_tracer() {}
