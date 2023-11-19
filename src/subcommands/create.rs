use crate::utils;
use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    // Dry run
    #[clap(long)]
    pub dry_run: bool,

    #[clap(long, value_name = "TEMPLATE")]
    pub template: Option<String>,
}

pub fn create_quiz() -> Result<()> {
    let quiz_path = utils::get_quiz_path()?;
    println!("quiz: {:?}", quiz_path.display());
    Ok(())
}

pub fn dry_run() -> Result<()> {
    let quiz_path = utils::get_quiz_path()?;
    println!("Dry run: quiz path would be {:?}", quiz_path.display());
    Ok(())
}
