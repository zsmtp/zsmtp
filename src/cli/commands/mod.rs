use clap::{
    Arg, ArgAction, Command,
    builder::styling::{AnsiColor, Effects, Styles},
};

#[allow(clippy::doc_markdown)]
pub mod built_info {
    include!(concat!(env!("OUT_DIR"), "/built.rs"));
}

#[must_use]
pub fn new() -> Command {
    let styles = Styles::styled()
        .header(AnsiColor::Yellow.on_default() | Effects::BOLD)
        .usage(AnsiColor::Green.on_default() | Effects::BOLD)
        .literal(AnsiColor::Blue.on_default() | Effects::BOLD)
        .placeholder(AnsiColor::Green.on_default());

    let git_hash = built_info::GIT_COMMIT_HASH.unwrap_or("unknown");
    let long_version: &'static str =
        Box::leak(format!("{} - {}", env!("CARGO_PKG_VERSION"), git_hash).into_boxed_str());

    Command::new(env!("CARGO_PKG_NAME"))
        .version(env!("CARGO_PKG_VERSION"))
        .long_version(long_version)
        .author(env!("CARGO_PKG_AUTHORS"))
        .about(env!("CARGO_PKG_DESCRIPTION"))
        .long_about(
            "A zero-knowledge SMTP mail transfer agent prototype.\n\n\
             EXAMPLES:\n  \
             zsmtp serve --config ./examples/zsmtp.yaml\n    \
             Start the placeholder relay service\n\n  \
             zsmtp submit --from alice@example.net --to bob@example.net --message-file mail.eml\n    \
             Build a prototype submission request and return a placeholder queue id\n\n  \
             zsmtp config show\n    \
             Print the resolved configuration as YAML\n\n  \
             zsmtp doctor\n    \
             Check configuration resolution and local filesystem expectations",
        )
        .styles(styles)
        .arg_required_else_help(true)
        .subcommand_required(true)
        .arg(arg_verbose())
        .arg(arg_config())
        .subcommand(cmd_serve())
        .subcommand(cmd_submit())
        .subcommand(cmd_config())
        .subcommand(cmd_doctor())
}

fn arg_verbose() -> Arg {
    Arg::new("verbose")
        .short('v')
        .long("verbose")
        .help("Increase log verbosity")
        .long_help(
            "Increase log verbosity.\n\n\
             -v   = INFO\n\
             -vv  = DEBUG\n\
             -vvv = TRACE",
        )
        .global(true)
        .action(ArgAction::Count)
}

fn arg_config() -> Arg {
    Arg::new("config")
        .short('C')
        .long("config")
        .value_name("PATH")
        .help("Path to a YAML configuration file")
        .global(true)
}

fn cmd_serve() -> Command {
    Command::new("serve").about("Start the placeholder SMTP relay service")
}

fn cmd_submit() -> Command {
    Command::new("submit")
        .about("Submit a message into the prototype queue")
        .arg(arg_from())
        .arg(arg_to())
        .arg(arg_message_file())
        .arg(arg_stdin())
}

fn cmd_config() -> Command {
    Command::new("config")
        .about("Inspect or validate configuration")
        .subcommand_required(true)
        .arg_required_else_help(true)
        .subcommand(Command::new("show").about("Print the resolved configuration"))
        .subcommand(Command::new("validate").about("Validate the resolved configuration"))
}

fn cmd_doctor() -> Command {
    Command::new("doctor").about("Run local configuration and environment checks")
}

fn arg_from() -> Arg {
    Arg::new("from")
        .long("from")
        .value_name("ADDR")
        .help("Envelope sender address")
        .required(true)
}

fn arg_to() -> Arg {
    Arg::new("to")
        .long("to")
        .value_name("ADDR")
        .help("Envelope recipient address")
        .required(true)
        .action(ArgAction::Append)
}

fn arg_message_file() -> Arg {
    Arg::new("message-file")
        .long("message-file")
        .value_name("PATH")
        .help("Read a raw RFC822 message from a file")
        .required_unless_present("stdin")
        .conflicts_with("stdin")
}

fn arg_stdin() -> Arg {
    Arg::new("stdin")
        .long("stdin")
        .help("Read a raw RFC822 message from standard input")
        .action(ArgAction::SetTrue)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn cli_has_expected_name() {
        let cmd = new();
        assert_eq!(cmd.get_name(), env!("CARGO_PKG_NAME"));
    }

    #[test]
    fn parses_submit_arguments() {
        let matches = new().get_matches_from(vec![
            "zsmtp",
            "submit",
            "--from",
            "alice@example.net",
            "--to",
            "bob@example.net",
            "--message-file",
            "mail.eml",
        ]);

        assert_eq!(matches.subcommand_name(), Some("submit"));
        let Some(("submit", submit_matches)) = matches.subcommand() else {
            unreachable!();
        };
        assert_eq!(
            submit_matches.get_one::<String>("from").map(String::as_str),
            Some("alice@example.net")
        );
    }

    #[test]
    fn rejects_submit_without_source() {
        let result = new().try_get_matches_from(vec![
            "zsmtp",
            "submit",
            "--from",
            "alice@example.net",
            "--to",
            "bob@example.net",
        ]);

        assert!(result.is_err());
    }

    #[test]
    fn parses_config_show_subcommand() {
        let matches = new().get_matches_from(vec!["zsmtp", "config", "show"]);
        let Some(("config", nested)) = matches.subcommand() else {
            unreachable!();
        };
        assert_eq!(nested.subcommand_name(), Some("show"));
    }
}
