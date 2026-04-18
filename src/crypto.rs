use crate::config::CryptoConfig;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CryptoContext {
    key_file: String,
    envelope_scheme: String,
    proof_system: String,
}

impl CryptoContext {
    #[must_use]
    pub fn from_settings(config: &CryptoConfig) -> Self {
        Self {
            key_file: config.key_file.display().to_string(),
            envelope_scheme: config.envelope_scheme.clone(),
            proof_system: config.proof_system.clone(),
        }
    }

    #[must_use]
    pub fn envelope_scheme(&self) -> &str {
        &self.envelope_scheme
    }

    #[must_use]
    pub fn proof_system(&self) -> &str {
        &self.proof_system
    }

    #[must_use]
    pub fn key_file(&self) -> &str {
        &self.key_file
    }
}
