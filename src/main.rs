use std::fs;
use std::path::PathBuf;

use once_cell::sync::Lazy;
use structopt::StructOpt;
use tera::{Context, Result, Tera};

// templates folder
const TEMPLATES_DIR: &str = "templates/**/*";

#[derive(Debug, StructOpt)]
#[structopt(name = "qdo", about = "quiz generator")]
struct Opt {
    /// quiuz title
    #[structopt(short = "t", long = "title")]
    title: String,

    /// template file in templates folder
    #[structopt(parse(from_os_str), short = "f", long = "file")]
    template: PathBuf,

    /// Output file, stdout if not present
    #[structopt(parse(from_os_str), short = "o", long = "out")]
    output: Option<PathBuf>,
}

static TEMPLATES: Lazy<Tera> = Lazy::new(|| {
    let mut tera = match Tera::new(TEMPLATES_DIR) {
        Ok(t) => t,
        Err(e) => {
            println!("Parsing error(s): {}", e);
            ::std::process::exit(1);
        }
    };
    tera.autoescape_on(vec!["html", ".sql"]);
    tera
});

fn main() -> Result<()> {
    let mut context = Context::new();

    let args = Opt::from_args();
    println!("{:?}", args);
    let title = "Sample Quiz";
    context.insert("title", &title);

    let templat_file_name = "quiz.html";
    let quiz = TEMPLATES.render(templat_file_name, &context)?;

    let html_file_name = "data/sample.html";
    let _ = fs::write(html_file_name, quiz)?;

    Ok(())
}
