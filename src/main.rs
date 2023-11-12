mod subcommands;
use subcommands::init::InitArgs;

use anyhow::Result;

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
        Commands::Init(init_args) => {
            if (init_args.dry_run) {
                subcommands::init::dry_run();
            } else {
                // TODO: Write actual code here
            }
        }
    }

    Ok(())
}
