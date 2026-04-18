use crate::cli::actions::ActionOutput;
use crate::config;
use crate::server::Server;
use anyhow::Result;
use std::path::Path;

/// Execute the placeholder `serve` action.
///
/// # Errors
///
/// Returns an error if configuration resolution or validation fails.
pub fn execute(config_path: Option<&Path>) -> Result<ActionOutput> {
    let loaded = config::load(config_path)?;
    let server = Server::new(loaded.config.server.clone());

    Ok(ActionOutput::new(format!(
        "zsmtp serve skeleton\nconfig_source: {}\nbind_address: {}\nhostname: {}\nstatus: {}",
        loaded.source.describe(),
        server.bind_address(),
        server.hostname(),
        server.status_message()
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn serve_returns_placeholder_output() {
        let output = execute(None);
        assert!(output.is_ok());
        if let Ok(output) = output {
            assert!(output.stdout.contains("zsmtp serve skeleton"));
        }
    }
}
