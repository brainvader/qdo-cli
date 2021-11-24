use std::{
    fs::{self, File},
    io::{stdout, BufWriter, Write},
    path::{Path, PathBuf},
    str::FromStr,
};

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
    let dry_run = args.dry_run;

    let app_name = env!("CARGO_PKG_NAME");
    let version = env!("CARGO_PKG_VERSION");
    let generator_name = [app_name, version].join(" ");

    // generate time stamp from the current time
    let utc: DateTime<Utc> = Utc::now();
    let year = utc.year().to_string();
    let month = utc.month().to_string();
    let day = utc.day().to_string();
    let time = utc.time().format("%H%M%S").to_string();

    // Create template context
    let mut context = tera::Context::new();
    context.insert("title", &title);
    context.insert("generator", &generator_name);
    context.insert("timestamp", &utc.format("%F%TZ").to_string());

    // Render template with context into html
    let quiz_html = Tera::one_off(template_str, &context, true)
        .with_context(|| format!("Fail to render template"))?;

    let file_name = PathBuf::from_str(&time_stamp)?;

    if dry_run {
        // Get stdout
        let out = stdout();

        // Setup buffer writer
        let mut writer = BufWriter::new(out.lock());
        writeln!(writer, "{}", &quiz_html).with_context(|| format!("Fail to write quiz"))?;
        let output = file_name.with_extension("html");
        writeln!(writer, "Above content saved to {:#?}", output.display())?;
        return Ok(());
    }

    // save template under quiz
    let quiz_dir = Path::new("./quiz");
    fs::create_dir_all(quiz_dir)
        .with_context(|| format!("failed to create {:#?}", quiz_dir.display()))?;
    let output = quiz_dir.join(file_name.with_extension("html"));
    let mut file = File::create(&output)?;
    file.write_all(quiz_html.as_bytes())
        .with_context(|| format!("faile to write, {:#?}", &output.file_name()))?;

    Ok(())
}
