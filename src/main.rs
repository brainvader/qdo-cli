use std::io::{stdout, BufWriter, Write};

use anyhow::{Context, Result};
use rust_embed::RustEmbed;
use structopt::StructOpt;
use tera::Tera;

#[derive(RustEmbed)]
#[folder = "templates/"]
struct Asset;

#[derive(Debug, StructOpt)]
#[structopt(name = "qdo", about = "quiz generator")]
struct Opt {
    /// quiuz title
    #[structopt(short = "t", long = "title")]
    title: String,
}

const ASSET_NAME: &str = "quiz.html";

fn main() -> Result<()> {
    // Read asset, quiz.html
    let asset =
        Asset::get(ASSET_NAME).with_context(|| format!("Fail to get asset: {}", ASSET_NAME))?;

    // Get a reference to data in COW struct
    let asset_data = asset.data.as_ref();

    // Get html string
    let html_str = std::str::from_utf8(asset_data)
        .with_context(|| format!("Fail to convert byte slice to string slice"))?;

    // Parse arguments
    let args = Opt::from_args();
    println!("{:?}", args);
    let title = "Sample Quiz";

    // Create template context
    let mut context = tera::Context::new();
    context.insert("title", &title);

    // Render template with context into html
    let quiz = Tera::one_off(html_str, &context, true)
        .with_context(|| format!("Fail to render template"))?;

    // Get stdout
    let out = stdout();

    // Setup buffer writer
    let mut writer = BufWriter::new(out.lock());
    writeln!(writer, "{}", &quiz).with_context(|| format!("Fail to write quiz"))?;

    Ok(())
}
