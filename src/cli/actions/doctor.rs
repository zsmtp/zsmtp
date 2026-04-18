use crate::cli::actions::ActionOutput;
use crate::config;
use anyhow::Result;
use std::fmt::Write as _;
use std::path::Path;

/// Execute local environment diagnostics for the placeholder deployment.
///
/// # Errors
///
/// Returns an error if configuration loading or validation fails.
pub fn execute(config_path: Option<&Path>) -> Result<ActionOutput> {
    let loaded = config::load(config_path)?;
    loaded.config.validate()?;

    let spool_dir = &loaded.config.server.spool_dir;
    let queue_dir = &loaded.config.submit.queue_dir;
    let key_file = &loaded.config.crypto.key_file;

    let mut report = String::new();
    let _ = writeln!(report, "zsmtp doctor report");
    let _ = writeln!(report, "config_source: {}", loaded.source.describe());
    let _ = writeln!(
        report,
        "spool_dir_exists: {} ({})",
        spool_dir.exists(),
        spool_dir.display()
    );
    let _ = writeln!(
        report,
        "queue_dir_exists: {} ({})",
        queue_dir.exists(),
        queue_dir.display()
    );
    let _ = writeln!(
        report,
        "key_file_exists: {} ({})",
        key_file.exists(),
        key_file.display()
    );
    let _ = writeln!(report, "status: skeleton-ready");

    Ok(ActionOutput::new(report.trim_end().to_owned()))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn doctor_reports_status() {
        let output = execute(None);
        assert!(output.is_ok());
        if let Ok(output) = output {
            assert!(output.stdout.contains("status: skeleton-ready"));
        }
    }
}
