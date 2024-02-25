use std::fs::{canonicalize, create_dir_all};
use std::path::PathBuf;

use anyhow::{anyhow, Context, Ok, Result};
use clap::Parser;
use log::info;
use tera::Tera;

use crate::utils::{self, get_quiz_template, GakuNote, TimeStamp};

#[derive(Parser)]
pub struct Args {
    // Dry run
    #[clap(long)]
    pub dry_run: bool,

    #[clap(long, value_name = "TEMPLATE")]
    pub template: Option<String>,

    #[clap(long)]
    path: Option<String>,
}

pub fn create_quiz(args: Args) -> Result<()> {
    let Args {
        dry_run,
        template,
        path,
    } = args;

    let timestamp: TimeStamp = Default::default();

    // TODO: extract the following match into a function
    let home = match path {
        Some(path) => {
            // For the path user given
            let custom_home = PathBuf::from(path);
            Ok(custom_home)
        }
        None => {
            // For default path (~/.qdo)
            let home_dir =
                utils::get_home_dir().with_context(|| "Failed to obtain the usr home directory")?;
            let qdo_path = PathBuf::from(home_dir).join(".qdo");

            let TimeStamp {
                year, month, day, ..
            } = timestamp;

            let _ = qdo_path
                .join(year.to_string())
                .join(month.to_string())
                .join(day.to_string());

            let default_home = utils::get_quiz_directory_path(&qdo_path)?;
            Ok(default_home)
        }
    }
    .with_context(|| "Failed to get a full path to quiz")?;

    // Run only when there are no directories
    if !home.exists() {
        create_dir_all(&home).with_context(|| {
            anyhow!(
                "Failed to create necessary directories: {}",
                &home.display()
            )
        })?
    }

    let home = canonicalize(&home).with_context(|| "Failed to get a absoluete path")?;
    let gaku_note = GakuNote::new(home, timestamp);

    if dry_run {
        info!("Dry run: quiz path would be {}", gaku_note.home.display());
        return Ok(());
    }

    // Generate note html content
    let mut context = tera::Context::new();
    context.insert("generator", &gaku_note.generator_name());
    let time_iso8601 = gaku_note.timestamp.iso_8601_format();
    context.insert("timestamp", &time_iso8601);
    println!("timestamp {}", time_iso8601);

    let template = match template {
        Some(template) => {
            let custom_template =
                get_quiz_template(&template).with_context(|| "Failed to get a custom template")?;
            Ok(custom_template)
        }
        None => {
            let default_template = utils::get_default_quiz_template()
                .with_context(|| "Failed to get a default quiz template")?;
            Ok(default_template)
        }
    }
    .with_context(|| "Failed to get template")?;

    let node_content = Tera::one_off(&template, &context, true)
        .with_context(|| anyhow!("Fail to render template"))?;

    gaku_note.save(&node_content)?;

    Ok(())
}
