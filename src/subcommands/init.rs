use anyhow::{Context, Result};
use clap::Parser;
use std::env;

#[derive(Parser)]
pub struct InitArgs {
    // Dry run
    #[clap(long)]
    pub dry_run: bool,
}

pub fn initialize_project(args: InitArgs) {
    println!("init");
}

pub fn dry_run() -> Result<()> {
    let home_dir =
        env::var("HOME").with_context(|| "Failed to retrieve HOME environment variable")?;
    println!("Create {}/qdo", home_dir);
    Ok(())
}
