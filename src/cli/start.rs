use crate::cli::{actions::Action, commands, dispatch, telemetry};
use anyhow::Result;

const fn get_verbosity_level(verbose_count: u8) -> Option<tracing::Level> {
    match verbose_count {
        0 => None,
        1 => Some(tracing::Level::INFO),
        2 => Some(tracing::Level::DEBUG),
        _ => Some(tracing::Level::TRACE),
    }
}

/// Parse CLI arguments, initialize telemetry, and return the typed action.
///
/// # Errors
///
/// Returns an error if telemetry initialization fails or the parsed arguments
/// cannot be dispatched into a valid action.
pub fn start() -> Result<Action> {
    let matches = commands::new().get_matches();
    let verbosity_level = get_verbosity_level(matches.get_count("verbose"));
    telemetry::init(verbosity_level)?;
    dispatch::handler(&matches)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn maps_verbosity_to_tracing_level() {
        assert_eq!(get_verbosity_level(0), None);
        assert_eq!(get_verbosity_level(1), Some(tracing::Level::INFO));
        assert_eq!(get_verbosity_level(2), Some(tracing::Level::DEBUG));
        assert_eq!(get_verbosity_level(3), Some(tracing::Level::TRACE));
        assert_eq!(get_verbosity_level(4), Some(tracing::Level::TRACE));
    }
}
