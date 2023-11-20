mod subcommands;
mod utils;

use anyhow::{Context, Ok, Result};
use clap::{command, Parser, Subcommand};

use subcommands::create;
use subcommands::init::InitArgs;

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
            subcommands::create::create_quiz(args).with_context(|| "Failed to create quiz")?;
        }
    }

    Ok(())
}
