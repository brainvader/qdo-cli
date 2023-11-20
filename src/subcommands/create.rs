use std::fs::{create_dir_all, File};
use std::io::Write;

use anyhow::{anyhow, Context, Ok, Result};
use clap::Parser;
use tera::Tera;

use crate::utils::{self, gen_timestamp, get_quiz_template, quiz_uuid};

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
    let Args { dry_run, template } = args;

    let uuid = utils::quiz_uuid();
    let file_name = format!("{}.html", uuid);

    let timestamp = utils::gen_timestamp().with_context(|| "Failed to generate timestamp")?;

    let quiz_path = match path {
        Some(path) => {
            let mut path = PathBuf::from(path);
            path.set_file_name(file_name);
            Ok(path)
        }
        None => {
            let mut path = utils::get_quiz_directory_path(&timestamp)?;
            path.set_file_name(file_name);
            Ok(path)
        }
    }
    .with_context(|| "Failed to get a full path to quiz")?;

    let app_name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let generator_name = [app_name, version].join(" ");

    let mut context = tera::Context::new();
    context.insert("generator", &generator_name);
    let time_iso8601 = timestamp.iso_8601_format();
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

    let quiz_html = Tera::one_off(&template, &context, true)
        .with_context(|| anyhow!("Fail to render template"))?;

    // Run only when there no directories
    if let Some(parent) = quiz_path.parent() {
        create_dir_all(parent).with_context(|| {
            anyhow!(
                "Failed to create necessary directories: {}",
                parent.display()
            )
        })?
    }

    let mut file = File::create(&quiz_path)
        .with_context(|| anyhow!("Failed to create quiz file {}", quiz_path.display()))?;
    file.write_all(quiz_html.as_bytes())
        .with_context(|| anyhow!("Failed to write quiz HTML to file: {}", quiz_path.display()))?;
    println!("{}", quiz_path.display());

    Ok(())
}

pub fn dry_run() -> Result<()> {
    let uuid = quiz_uuid();
    let timestamp = gen_timestamp().with_context(|| "Failed to generate timestamp")?;
    let quiz_path = utils::get_quiz_path(&uuid, &timestamp)?;
    println!("Dry run: quiz path would be {:?}", quiz_path.display());
    Ok(())
}
