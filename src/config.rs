use crate::error::ConfigError;
use anyhow::{Context, Result};
use serde::{Deserialize, Serialize};
use std::env;
use std::fs;
use std::path::{Path, PathBuf};

const ENV_CONFIG_KEY: &str = "ZSMTP_CONFIG";
const DEFAULT_CONFIG_PATHS: [&str; 2] = ["./zsmtp.yaml", "/etc/zsmtp/zsmtp.yaml"];

#[derive(Debug, Clone, Default, Serialize, Deserialize, PartialEq, Eq)]
pub struct AppConfig {
    #[serde(default)]
    pub server: ServerConfig,
    #[serde(default)]
    pub submit: SubmitConfig,
    #[serde(default)]
    pub protocol: ProtocolConfig,
    #[serde(default)]
    pub crypto: CryptoConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ServerConfig {
    #[serde(default = "default_bind_address")]
    pub bind_address: String,
    #[serde(default = "default_hostname")]
    pub hostname: String,
    #[serde(default = "default_spool_dir")]
    pub spool_dir: PathBuf,
    #[serde(default = "default_max_message_size")]
    pub max_message_size: usize,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct SubmitConfig {
    #[serde(default = "default_queue_dir")]
    pub queue_dir: PathBuf,
    #[serde(default = "default_allow_stdin")]
    pub allow_stdin: bool,
    #[serde(default = "default_sender_domain")]
    pub default_sender_domain: String,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct ProtocolConfig {
    #[serde(default = "default_protocol_profile")]
    pub profile: String,
    #[serde(default = "default_require_starttls")]
    pub require_starttls: bool,
    #[serde(default = "default_zero_knowledge")]
    pub zero_knowledge: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub struct CryptoConfig {
    #[serde(default = "default_key_file")]
    pub key_file: PathBuf,
    #[serde(default = "default_envelope_scheme")]
    pub envelope_scheme: String,
    #[serde(default = "default_proof_system")]
    pub proof_system: String,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LoadedConfig {
    pub source: ConfigSource,
    pub config: AppConfig,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ConfigSource {
    Explicit(PathBuf),
    Environment(PathBuf),
    DefaultPath(PathBuf),
    BuiltInDefault,
}

impl ConfigSource {
    #[must_use]
    pub fn describe(&self) -> String {
        match self {
            Self::Explicit(path) => format!("explicit:{}", path.display()),
            Self::Environment(path) => format!("env:{}={}", ENV_CONFIG_KEY, path.display()),
            Self::DefaultPath(path) => format!("default-path:{}", path.display()),
            Self::BuiltInDefault => "built-in-default".to_string(),
        }
    }
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_address: default_bind_address(),
            hostname: default_hostname(),
            spool_dir: default_spool_dir(),
            max_message_size: default_max_message_size(),
        }
    }
}

impl Default for SubmitConfig {
    fn default() -> Self {
        Self {
            queue_dir: default_queue_dir(),
            allow_stdin: default_allow_stdin(),
            default_sender_domain: default_sender_domain(),
        }
    }
}

impl Default for ProtocolConfig {
    fn default() -> Self {
        Self {
            profile: default_protocol_profile(),
            require_starttls: default_require_starttls(),
            zero_knowledge: default_zero_knowledge(),
        }
    }
}

impl Default for CryptoConfig {
    fn default() -> Self {
        Self {
            key_file: default_key_file(),
            envelope_scheme: default_envelope_scheme(),
            proof_system: default_proof_system(),
        }
    }
}

impl AppConfig {
    /// Validate that the resolved configuration is internally consistent.
    ///
    /// # Errors
    ///
    /// Returns an error when required fields are empty or numeric limits are
    /// invalid.
    pub fn validate(&self) -> Result<()> {
        if self.server.bind_address.trim().is_empty() {
            return Err(ConfigError::EmptyField("server.bind_address").into());
        }
        if self.server.hostname.trim().is_empty() {
            return Err(ConfigError::EmptyField("server.hostname").into());
        }
        if self.server.max_message_size == 0 {
            return Err(ConfigError::ZeroValue("server.max_message_size").into());
        }
        if self.submit.default_sender_domain.trim().is_empty() {
            return Err(ConfigError::EmptyField("submit.default_sender_domain").into());
        }
        if self.protocol.profile.trim().is_empty() {
            return Err(ConfigError::EmptyField("protocol.profile").into());
        }
        if self.crypto.envelope_scheme.trim().is_empty() {
            return Err(ConfigError::EmptyField("crypto.envelope_scheme").into());
        }
        if self.crypto.proof_system.trim().is_empty() {
            return Err(ConfigError::EmptyField("crypto.proof_system").into());
        }
        Ok(())
    }
}

/// Resolve and load configuration from CLI, environment, filesystem, or defaults.
///
/// # Errors
///
/// Returns an error if an explicit or discovered configuration file cannot be
/// read, parsed, or validated.
pub fn load(config_path: Option<&Path>) -> Result<LoadedConfig> {
    if let Some(path) = config_path {
        return load_from_path(path, ConfigSource::Explicit(path.to_path_buf()));
    }

    if let Some(path) = env::var_os(ENV_CONFIG_KEY).map(PathBuf::from) {
        return load_from_path(&path, ConfigSource::Environment(path.clone()));
    }

    if let Some(path) = DEFAULT_CONFIG_PATHS
        .into_iter()
        .map(PathBuf::from)
        .find(|path| path.exists())
    {
        return load_from_path(&path, ConfigSource::DefaultPath(path.clone()));
    }

    let config = AppConfig::default();
    config.validate()?;
    Ok(LoadedConfig {
        source: ConfigSource::BuiltInDefault,
        config,
    })
}

fn load_from_path(path: &Path, source: ConfigSource) -> Result<LoadedConfig> {
    if !path.exists() {
        return Err(ConfigError::MissingFile(path.to_path_buf()).into());
    }

    let raw = fs::read_to_string(path)
        .with_context(|| format!("failed to read configuration file {}", path.display()))?;
    let config: AppConfig = serde_yaml::from_str(&raw)
        .with_context(|| format!("failed to parse YAML configuration {}", path.display()))?;
    config.validate()?;
    Ok(LoadedConfig { source, config })
}

fn default_bind_address() -> String {
    "127.0.0.1:2525".to_string()
}

fn default_hostname() -> String {
    "localhost".to_string()
}

fn default_spool_dir() -> PathBuf {
    PathBuf::from("./var/spool/zsmtp")
}

const fn default_max_message_size() -> usize {
    10 * 1024 * 1024
}

fn default_queue_dir() -> PathBuf {
    PathBuf::from("./var/queue/zsmtp")
}

const fn default_allow_stdin() -> bool {
    true
}

fn default_sender_domain() -> String {
    "local.test".to_string()
}

fn default_protocol_profile() -> String {
    "zk-smtp-draft-0".to_string()
}

const fn default_require_starttls() -> bool {
    false
}

const fn default_zero_knowledge() -> bool {
    true
}

fn default_key_file() -> PathBuf {
    PathBuf::from("./var/lib/zsmtp/server.key")
}

fn default_envelope_scheme() -> String {
    "hybrid-envelope".to_string()
}

fn default_proof_system() -> String {
    "placeholder-proof".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;
    use std::sync::{Mutex, MutexGuard};
    use tempfile::TempDir;

    static ENV_LOCK: Mutex<()> = Mutex::new(());

    fn env_guard() -> MutexGuard<'static, ()> {
        ENV_LOCK
            .lock()
            .unwrap_or_else(std::sync::PoisonError::into_inner)
    }

    #[test]
    #[allow(clippy::undocumented_unsafe_blocks)]
    fn load_returns_default_when_no_file_exists() {
        let _guard = env_guard();
        unsafe {
            env::remove_var(ENV_CONFIG_KEY);
        }
        let loaded = load(None);
        assert!(loaded.is_ok());
        if let Ok(loaded) = loaded {
            assert_eq!(loaded.source, ConfigSource::BuiltInDefault);
        }
    }

    #[test]
    fn load_reads_yaml_from_explicit_path() {
        let tempdir = TempDir::new();
        assert!(tempdir.is_ok());
        let Ok(tempdir) = tempdir else {
            unreachable!();
        };

        let config_path = tempdir.path().join("zsmtp.yaml");
        let write_result = fs::write(
            &config_path,
            "server:\n  bind_address: \"127.0.0.1:2525\"\n  hostname: \"mx.local\"\n",
        );
        assert!(write_result.is_ok());

        let loaded = load(Some(&config_path));
        assert!(loaded.is_ok());
        if let Ok(loaded) = loaded {
            assert_eq!(loaded.source, ConfigSource::Explicit(config_path));
            assert_eq!(loaded.config.server.hostname, "mx.local");
        }
    }

    #[test]
    fn missing_explicit_path_is_an_error() {
        let missing = PathBuf::from("/tmp/definitely-missing-zsmtp-config.yaml");
        let loaded = load(Some(&missing));
        assert!(loaded.is_err());
    }

    #[test]
    #[allow(clippy::undocumented_unsafe_blocks)]
    fn env_override_is_used() {
        let _guard = env_guard();
        let tempdir = TempDir::new();
        assert!(tempdir.is_ok());
        let Ok(tempdir) = tempdir else {
            unreachable!();
        };

        let config_path = tempdir.path().join("env.yaml");
        let write_result = fs::write(
            &config_path,
            "server:\n  bind_address: \"127.0.0.1:2626\"\nsubmit:\n  default_sender_domain: \"example.test\"\n",
        );
        assert!(write_result.is_ok());

        unsafe {
            env::set_var(ENV_CONFIG_KEY, &config_path);
        }

        let loaded = load(None);
        assert!(loaded.is_ok());
        if let Ok(loaded) = loaded {
            assert_eq!(loaded.source, ConfigSource::Environment(config_path));
            assert_eq!(loaded.config.submit.default_sender_domain, "example.test");
        }

        unsafe {
            env::remove_var(ENV_CONFIG_KEY);
        }
    }
}
