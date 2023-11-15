use crate::utils;
use anyhow::{Context, Result};
use clap::Parser;

#[derive(Parser)]
pub struct Args {
    // Dry run
    #[clap(long)]
    pub dry_run: bool,
}

pub fn create_quiz() -> Result<()> {
    let uuid = utils::quiz_uuid();
    let (year, month, day, time) = utils::gen_timestamp();
    let qdo_path = utils::get_qdo_path().with_context(|| "Failed to full path to qdo dir")?;
    let mut quiz_path = qdo_path
        .join(year.to_string())
        .join(month.to_string())
        .join(day.to_string())
        .join(uuid.to_string());
    quiz_path.set_extension("html");
    println!("quiz: {:?}", quiz_path.display());
    Ok(())
}

pub fn dry_run() -> Result<()> {
    let uuid = utils::quiz_uuid();
    let (year, month, day, time) = utils::gen_timestamp();
    let qdo_path = utils::get_qdo_path().with_context(|| "Failed to full path to qdo dir")?;
    let mut quiz_path = qdo_path
        .join(year.to_string())
        .join(month.to_string())
        .join(day.to_string())
        .join(uuid.to_string());
    quiz_path.set_extension("html");
    println!("quiz: {:?}", quiz_path.display());
    Ok(())
}
