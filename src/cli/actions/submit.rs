use crate::cli::actions::ActionOutput;
use crate::config;
use crate::message::{MessageSource, SubmissionRequest};
use crate::protocol::ProtocolProfile;
use anyhow::Result;
use std::path::Path;

/// Execute the placeholder `submit` action.
///
/// # Errors
///
/// Returns an error if configuration loading fails or the message payload cannot
/// be read from the selected source.
pub fn execute(
    config_path: Option<&Path>,
    envelope_from: &str,
    recipients: &[String],
    source: &MessageSource,
) -> Result<ActionOutput> {
    let loaded = config::load(config_path)?;
    let raw_message = source.read_message()?;
    let request = SubmissionRequest::new(
        envelope_from.to_owned(),
        recipients.to_vec(),
        source.clone(),
        raw_message,
    );
    let profile = ProtocolProfile::from_settings(&loaded.config.protocol);
    let queue_id = request.placeholder_queue_id(&profile, &loaded.config.crypto);

    Ok(ActionOutput::new(format!(
        "submission accepted by prototype skeleton\nqueue_id: {queue_id}\nconfig_source: {}\nprotocol: {}\nrecipients: {}\nmessage_bytes: {}",
        loaded.source.describe(),
        profile.name(),
        request.recipients().len(),
        request.raw_message().len()
    )))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use tempfile::TempDir;

    #[test]
    fn submit_returns_deterministic_queue_id() {
        let tempdir = TempDir::new();
        assert!(tempdir.is_ok());
        let Ok(tempdir) = tempdir else {
            unreachable!();
        };

        let message_path = tempdir.path().join("mail.eml");
        let write_result = fs::write(
            &message_path,
            "From: alice@example.net\nTo: bob@example.net\n\nhello\n",
        );
        assert!(write_result.is_ok());

        let recipients = vec!["bob@example.net".to_string()];
        let output = execute(
            None,
            "alice@example.net",
            &recipients,
            &MessageSource::File(message_path),
        );
        assert!(output.is_ok());
        if let Ok(output) = output {
            assert!(output.stdout.contains("queue_id: stub-"));
        }
    }
}
