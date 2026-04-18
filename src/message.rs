use crate::config::CryptoConfig;
use crate::protocol::ProtocolProfile;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::fs;
use std::io::{self, Read};
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum MessageSource {
    File(PathBuf),
    Stdin,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct SubmissionRequest {
    envelope_from: String,
    recipients: Vec<String>,
    source: MessageSource,
    raw_message: String,
}

impl SubmissionRequest {
    #[must_use]
    pub fn new(
        envelope_from: String,
        recipients: Vec<String>,
        source: MessageSource,
        raw_message: String,
    ) -> Self {
        Self {
            envelope_from,
            recipients,
            source,
            raw_message,
        }
    }

    #[must_use]
    pub fn recipients(&self) -> &[String] {
        &self.recipients
    }

    #[must_use]
    pub fn raw_message(&self) -> &str {
        &self.raw_message
    }

    #[must_use]
    pub fn placeholder_queue_id(&self, profile: &ProtocolProfile, crypto: &CryptoConfig) -> String {
        let mut hasher = blake3::Hasher::new();
        hasher.update(self.envelope_from.as_bytes());
        hasher.update(self.recipients.join(",").as_bytes());
        hasher.update(profile.name().as_bytes());
        hasher.update(crypto.envelope_scheme.as_bytes());
        hasher.update(crypto.proof_system.as_bytes());
        hasher.update(self.raw_message.as_bytes());
        let hash = hasher.finalize().to_hex().to_string();
        format!("stub-{}", &hash[..12])
    }

    #[must_use]
    pub fn source(&self) -> &MessageSource {
        &self.source
    }
}

impl MessageSource {
    /// Read the raw message payload from the selected source.
    ///
    /// # Errors
    ///
    /// Returns an error if the file cannot be read or stdin consumption fails.
    pub fn read_message(&self) -> Result<String> {
        match self {
            Self::File(path) => fs::read_to_string(path)
                .with_context(|| format!("failed to read message file {}", path.display())),
            Self::Stdin => {
                let mut buffer = String::new();
                io::stdin()
                    .read_to_string(&mut buffer)
                    .context("failed to read message from stdin")?;
                Ok(buffer)
            }
        }
    }
}
