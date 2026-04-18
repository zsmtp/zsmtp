pub mod config;
pub mod doctor;
pub mod serve;
pub mod submit;

use crate::message::MessageSource;
use std::path::PathBuf;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum Action {
    Serve {
        config_path: Option<PathBuf>,
        verbose: bool,
    },
    Submit {
        config_path: Option<PathBuf>,
        verbose: bool,
        envelope_from: String,
        recipients: Vec<String>,
        source: MessageSource,
    },
    ConfigShow {
        config_path: Option<PathBuf>,
        verbose: bool,
    },
    ConfigValidate {
        config_path: Option<PathBuf>,
        verbose: bool,
    },
    Doctor {
        config_path: Option<PathBuf>,
        verbose: bool,
    },
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ActionOutput {
    pub stdout: String,
}

impl ActionOutput {
    #[must_use]
    pub fn new(stdout: impl Into<String>) -> Self {
        Self {
            stdout: stdout.into(),
        }
    }
}
