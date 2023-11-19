mod subcommands;
mod utils;

use subcommands::create;
use subcommands::init::InitArgs;

use anyhow::{Context, Ok, Result};

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
    Create(create::Args),
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
        Commands::Create(args) => {
            if args.dry_run {
                subcommands::create::dry_run()
                    .with_context(|| "Failed to call qdo create --dry_run")?;
                return Ok(());
            }

            subcommands::create::create_quiz(args).with_context(|| "Failed to create quiz")?;
        }
    }

    Ok(())
}
