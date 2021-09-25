use std::error::Error;
use std::result::Result;

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

fn main() -> Result<(), Box<dyn Error>> {
    let mut context = tera::Context::new();
    let quiz_template = Asset::get("quiz.html").unwrap();
    let quiz_html = std::str::from_utf8(quiz_template.data.as_ref()).unwrap();

    let args = Opt::from_args();
    println!("{:?}", args);
    let title = "Sample Quiz";
    context.insert("title", &title);

    let quiz = Tera::one_off(quiz_html, &context, true).unwrap();

    println!("{}", quiz);

    Ok(())
}
