use anyhow::{Context, Result};
use clap::Parser;
use std::path::PathBuf;
use std::{env, fs};

#[derive(Parser)]
pub struct InitArgs {
    // Dry run
    #[clap(long)]
    pub dry_run: bool,
}

fn get_home_dir() -> Result<String> {
    let home_dir =
        env::var("HOME").with_context(|| "Failed to retrieve HOME environment variable")?;
    Ok(home_dir)
}

fn gen_qdo_dir(home_dir: &str) -> Result<()> {
    let qdo_dir = PathBuf::from(home_dir).join("qdo");

    // Check if the qdo directory exists
    if !qdo_dir.exists() {
        // Create the directory if it does not exist
        fs::create_dir(&qdo_dir)?;
        println!("ディレクトリを作成しました: {:?}", qdo_dir);
    } else {
        // Inform if the directory already exists
        println!("ディレクトリは既に存在します: {:?}", qdo_dir);
    }

    Ok(())
}

pub fn initialize_project() -> Result<()> {
    let home_dir = get_home_dir().with_context(|| "Failed to obtain the home directory")?;
    gen_qdo_dir(&home_dir);
    Ok(())
}

pub fn dry_run() -> Result<()> {
    let home_dir = get_home_dir().with_context(|| "Failed to obtain the home directory")?;
    println!("Create {}/qdo", home_dir);
    Ok(())
}
