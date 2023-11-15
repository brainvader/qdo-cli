use crate::utils;
use anyhow::Result;
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    // Dry run
    #[clap(long)]
    pub dry_run: bool,
}

pub fn create_quiz() -> Result<()> {
    let uuid = utils::quiz_uuid();
    println!("quiz id: {}", uuid);
    Ok(())
}

pub fn dry_run() -> Result<()> {
    let uuid = utils::quiz_uuid();
    println!("quiz id: {}", uuid);
    Ok(())
}
