use std::fs;

use once_cell::sync::Lazy;
use tera::{Context, Result, Tera};

static TEMPLATES: Lazy<Tera> = Lazy::new(|| {
    let mut tera = match Tera::new("templates/**/*") {
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
    let title = "Sample Quiz";
    context.insert("title", &title);

    let templat_file_name = "quiz.html";
    let quiz = TEMPLATES.render(templat_file_name, &context)?;

    let html_file_name = "data/sample.html";
    let _ = fs::write(html_file_name, quiz)?;

    Ok(())
}
