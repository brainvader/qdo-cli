use crate::utils;
use anyhow::{Context, Result};
use clap::Parser;
use uuid::timestamp;

#[derive(Parser)]
pub struct Args {
    // Dry run
    #[clap(long)]
    pub dry_run: bool,
}

pub fn create_quiz() -> Result<()> {
    let uuid = utils::quiz_uuid();
    let (year, month, day, time) = utils::gen_timestamp();
    println!(
        "quiz {} is generated at {}-{}-{}-{}",
        uuid, year, month, day, time
    );
    Ok(())
}

pub fn dry_run() -> Result<()> {
    let uuid = utils::quiz_uuid();
    let (year, month, day, time) = utils::gen_timestamp();
    println!(
        "quiz {} is generated at {}-{}-{}-{}",
        uuid, year, month, day, time
    );
    Ok(())
}
