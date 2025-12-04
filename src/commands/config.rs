use {
    crate::{
        commands::CommandExec,
        config::{ScillaConfig, scilla_config_path},
        context::ScillaContext,
        error::ScillaResult,
    },
    console::style,
    std::{env, process::Command as StdCommand},
};

/// Commands related to configuration like RPC_URL, KEYPAIR_PATH etc
#[derive(Debug, Clone)]
pub enum ConfigCommand {
    Show,
    Generate,
    Edit,
    GoBack,
}

impl ConfigCommand {
    pub async fn process_command(&self, _ctx: &ScillaContext) -> ScillaResult<()> {
        match self {
            ConfigCommand::Show => {
                show_config()?;
            }
            ConfigCommand::Generate => {
                generate_config()?;
            }
            ConfigCommand::Edit => {
                edit_config()?;
            }
            ConfigCommand::GoBack => {
                return Ok(CommandExec::GoBack);
            }
        }
        Ok(CommandExec::Process(()))
    }
}

fn show_config() -> anyhow::Result<()> {
    let path = scilla_config_path();
    if !path.exists() {
        println!(
            "{}\n  Config file not found at: {}\n  Run {} to create one.",
            style("⚠ No config found").yellow().bold(),
            style(path.display()).cyan(),
            style("Generate ScillaConfig").green()
        );
        return Ok(());
    }

    let config = ScillaConfig::load()?;
    println!("{}", style("Current Scilla Config").green().bold());
    println!("  RPC URL:          {}", style(&config.rpc_url).cyan());
    println!(
        "  Keypair Path:     {}",
        style(config.keypair_path.display()).cyan()
    );
    println!(
        "  Commitment Level: {}",
        style(format!("{:?}", config.commitment_level)).cyan()
    );
    println!("\n  Config path: {}", style(path.display()).dim());
    Ok(())
}

fn generate_config() -> anyhow::Result<()> {
    let path = scilla_config_path();
    if path.exists() {
        println!(
            "{} Config already exists at: {}\n  Use {} to view or {} to modify.",
            style("⚠").yellow(),
            style(path.display()).cyan(),
            style("Show ScillaConfig").green(),
            style("Edit ScillaConfig").green()
        );
        return Ok(());
    }

    let default_config = ScillaConfig::default();
    default_config.save()?;

    println!(
        "{} Config created at: {}",
        style("✅").green(),
        style(path.display()).cyan()
    );
    Ok(())
}

fn edit_config() -> anyhow::Result<()> {
    let path = scilla_config_path();
    if !path.exists() {
        println!(
            "{} Config file does not exist. Run {} first.",
            style("⚠").yellow(),
            style("Generate ScillaConfig").green()
        );
        return Ok(());
    }

    if let Ok(editor) = env::var("EDITOR") {
        StdCommand::new(&editor).arg(&path).status()?;
        println!(
            "{} Config may have been modified. Restart Scilla to apply changes.",
            style("ℹ").blue()
        );
    } else {
        println!(
            "{} $EDITOR not set. Open the config manually:\n  {}",
            style("ℹ").blue(),
            style(path.display()).cyan()
        );
    }
    Ok(())
}
