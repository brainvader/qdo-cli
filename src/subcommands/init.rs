use anyhow::{Context, Result};
use clap::Parser;

use crate::utils;

#[derive(Parser)]
pub struct InitArgs {
    // Dry run
    #[clap(long)]
    pub dry_run: bool,
}

pub fn initialize_project() -> Result<()> {
    let home_dir = utils::get_home_dir().with_context(|| "Failed to obtain the home directory")?;
    utils::gen_qdo_dir(&home_dir).with_context(|| "Failed to create a qdo directory")?;
    Ok(())
}

pub fn dry_run() -> Result<()> {
    let home_dir = utils::get_home_dir().with_context(|| "Failed to obtain the home directory")?;
    println!("Create {}/.qdo", home_dir);
    Ok(())
}
