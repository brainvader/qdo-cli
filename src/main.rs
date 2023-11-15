mod subcommands;
use subcommands::init::InitArgs;

use anyhow::{Context, Result};

use clap::{command, Parser, Subcommand};

#[derive(Parser)]
#[command(name = "qdo", about = "A quiz generator", version)]
pub struct Cli {
    #[command(subcommand)]
    pub commands: Commands,
}

#[derive(Subcommand)]
pub enum Commands {
    Init(InitArgs),
}

fn main() -> Result<()> {
    // Parse arguments
    let cli = Cli::parse();

    match cli.commands {
        Commands::Init(args) => {
            if args.dry_run {
                subcommands::init::dry_run().with_context(|| "Failed to call dry_run of init")?;
            } else {
                subcommands::init::initialize_project()
                    .with_context(|| "Failed to call initialize_project")?;
            }
        }
    }

    Ok(())
}
