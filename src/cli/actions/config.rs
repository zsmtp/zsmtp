use crate::cli::actions::ActionOutput;
use crate::config;
use anyhow::Result;
use std::path::Path;

/// Execute `config show` and print the resolved configuration as YAML.
///
/// # Errors
///
/// Returns an error if configuration loading or YAML serialization fails.
pub fn execute_show(config_path: Option<&Path>) -> Result<ActionOutput> {
    let loaded = config::load(config_path)?;
    let yaml = serde_yaml::to_string(&loaded.config)?;
    Ok(ActionOutput::new(format!(
        "# source: {}\n{}",
        loaded.source.describe(),
        yaml
    )))
}

/// Execute `config validate` against the resolved configuration.
///
/// # Errors
///
/// Returns an error if configuration loading or validation fails.
pub fn execute_validate(config_path: Option<&Path>) -> Result<ActionOutput> {
    let loaded = config::load(config_path)?;
    loaded.config.validate()?;
    Ok(ActionOutput::new(format!(
        "configuration valid\nsource: {}",
        loaded.source.describe()
    )))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn config_show_serializes_yaml() {
        let output = execute_show(None);
        assert!(output.is_ok());
        if let Ok(output) = output {
            assert!(output.stdout.contains("server:"));
        }
    }

    #[test]
    fn config_validate_accepts_defaults() {
        let output = execute_validate(None);
        assert!(output.is_ok());
    }
}
