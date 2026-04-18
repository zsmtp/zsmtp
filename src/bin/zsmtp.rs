use anyhow::Result;
use tracing::info;
use zsmtp::cli;

#[tokio::main(flavor = "current_thread")]
async fn main() -> Result<()> {
    let action = cli::start()?;

    info!("starting zsmtp execution");

    let output = match action {
        cli::actions::Action::Serve {
            config_path,
            verbose,
        } => {
            info!(action = "serve", verbose, "executing serve action");
            cli::actions::serve::execute(config_path.as_deref())?
        }
        cli::actions::Action::Submit {
            config_path,
            verbose,
            envelope_from,
            recipients,
            source,
        } => {
            info!(
                action = "submit",
                verbose,
                recipient_count = recipients.len(),
                "executing submit action"
            );
            cli::actions::submit::execute(
                config_path.as_deref(),
                &envelope_from,
                &recipients,
                &source,
            )?
        }
        cli::actions::Action::ConfigShow {
            config_path,
            verbose,
        } => {
            info!(action = "config-show", verbose, "executing config show");
            cli::actions::config::execute_show(config_path.as_deref())?
        }
        cli::actions::Action::ConfigValidate {
            config_path,
            verbose,
        } => {
            info!(
                action = "config-validate",
                verbose, "executing config validate"
            );
            cli::actions::config::execute_validate(config_path.as_deref())?
        }
        cli::actions::Action::Doctor {
            config_path,
            verbose,
        } => {
            info!(action = "doctor", verbose, "executing doctor action");
            cli::actions::doctor::execute(config_path.as_deref())?
        }
    };

    if !output.stdout.is_empty() {
        println!("{}", output.stdout);
    }

    cli::telemetry::shutdown_tracer();
    tokio::time::sleep(std::time::Duration::from_millis(50)).await;

    Ok(())
}
