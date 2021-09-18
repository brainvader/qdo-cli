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

fn main() -> Result<()>{
    let mut context = Context::new();
    context.insert("title", &"First Quiz");


    let quiz= TEMPLATES.render("quiz.html", &context)?;
    let _ = fs::write("data/sample.html", quiz)?;
    
    Ok(())
}
