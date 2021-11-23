use std::io::{stdout, BufWriter, Write};

use anyhow::{Context, Result};
use clap::{self, Parser};
use rust_embed::RustEmbed;
use tera::Tera;

use chrono::offset::Utc;

#[derive(RustEmbed)]
#[folder = "templates/"]
struct Asset;

#[derive(Debug, clap::Parser)]
#[clap(name = "qdo", about = "quiz generator")]
struct QdoArgs {
    /// quiuz title
    #[clap(short = 't', long = "title")]
    title: String,

    /// dry run
    #[clap(long)]
    dry_run: bool,
}

const ASSET_NAME: &str = "quiz.html";

fn main() -> Result<()> {
    // Read asset, quiz.html
    let asset =
        Asset::get(ASSET_NAME).with_context(|| format!("Fail to get asset: {}", ASSET_NAME))?;

    // Get a reference to data in COW struct
    let asset_data = asset.data.as_ref();

    // Get html string
    let template_str = std::str::from_utf8(asset_data)
        .with_context(|| format!("Fail to convert byte slice to string slice"))?;

    // Parse arguments
    let args = QdoArgs::parse();
    let title = &args.title;

    let app_name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let generator_name = [app_name, version].join(" ");

    let time_stamp: String = Utc::now().format("%FT%TZ").to_string();

    // Create template context
    let mut context = tera::Context::new();
    context.insert("title", &title);
    context.insert("generator", &generator_name);

    // Render template with context into html
    let quiz_html = Tera::one_off(template_str, &context, true)
        .with_context(|| format!("Fail to render template"))?;

    // Get stdout
    let out = stdout();

    // Setup buffer writer
    let mut writer = BufWriter::new(out.lock());
    writeln!(writer, "{}", &quiz_html).with_context(|| format!("Fail to write quiz"))?;

    Ok(())
}
