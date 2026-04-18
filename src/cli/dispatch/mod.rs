use crate::cli::actions::Action;
use crate::message::MessageSource;
use anyhow::{Context, Result, anyhow, bail};
use clap::ArgMatches;
use std::path::PathBuf;

/// Convert parsed clap matches into a typed CLI action.
///
/// # Errors
///
/// Returns an error when required arguments are missing or an unsupported
/// subcommand shape is encountered.
pub fn handler(matches: &ArgMatches) -> Result<Action> {
    let verbose = matches.get_count("verbose") > 0;
    let config_path = matches.get_one::<String>("config").map(PathBuf::from);

    match matches.subcommand() {
        Some(("serve", _)) => Ok(Action::Serve {
            config_path,
            verbose,
        }),
        Some(("submit", submit_matches)) => Ok(Action::Submit {
            config_path,
            verbose,
            envelope_from: required_string(submit_matches, "from")?,
            recipients: required_many_strings(submit_matches, "to")?,
            source: message_source(submit_matches)?,
        }),
        Some(("config", config_matches)) => match config_matches.subcommand_name() {
            Some("show") => Ok(Action::ConfigShow {
                config_path,
                verbose,
            }),
            Some("validate") => Ok(Action::ConfigValidate {
                config_path,
                verbose,
            }),
            Some(other) => bail!("unsupported config subcommand: {other}"),
            None => bail!("missing config subcommand"),
        },
        Some(("doctor", _)) => Ok(Action::Doctor {
            config_path,
            verbose,
        }),
        Some((other, _)) => bail!("unsupported subcommand: {other}"),
        None => bail!("no action selected"),
    }
}

fn required_string(matches: &ArgMatches, id: &str) -> Result<String> {
    matches
        .get_one::<String>(id)
        .cloned()
        .ok_or_else(|| anyhow!("missing required argument: {id}"))
}

fn required_many_strings(matches: &ArgMatches, id: &str) -> Result<Vec<String>> {
    matches
        .get_many::<String>(id)
        .map(|values| values.cloned().collect::<Vec<_>>())
        .context("missing required recipients")
}

fn message_source(matches: &ArgMatches) -> Result<MessageSource> {
    if matches.get_flag("stdin") {
        Ok(MessageSource::Stdin)
    } else if let Some(path) = matches.get_one::<String>("message-file") {
        Ok(MessageSource::File(PathBuf::from(path)))
    } else {
        bail!("message source must be provided via --message-file or --stdin")
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::cli::commands;

    #[test]
    fn dispatches_serve_action() {
        let matches = commands::new().get_matches_from(vec!["zsmtp", "serve"]);
        let action = handler(&matches);
        assert!(action.is_ok());
        if let Ok(Action::Serve { .. }) = action {
        } else {
            unreachable!();
        }
    }

    #[test]
    fn dispatches_submit_action() {
        let matches = commands::new().get_matches_from(vec![
            "zsmtp",
            "submit",
            "--from",
            "alice@example.net",
            "--to",
            "bob@example.net",
            "--message-file",
            "mail.eml",
        ]);
        let action = handler(&matches);
        assert!(action.is_ok());
        if let Ok(Action::Submit { recipients, .. }) = action {
            assert_eq!(recipients, vec!["bob@example.net".to_string()]);
        } else {
            unreachable!();
        }
    }

    #[test]
    fn dispatches_config_validate_action() {
        let matches = commands::new().get_matches_from(vec!["zsmtp", "config", "validate"]);
        let action = handler(&matches);
        assert!(action.is_ok());
        if let Ok(Action::ConfigValidate { .. }) = action {
        } else {
            unreachable!();
        }
    }
}
