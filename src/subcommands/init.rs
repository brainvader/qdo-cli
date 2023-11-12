use clap::Parser;

#[derive(Parser)]
pub struct InitArgs {
    pub name: String,

    // Dry run
    #[clap(long)]
    pub dry_run: bool,
}

pub fn initialize_project(args: InitArgs) {
    println!("init");
}

pub fn dry_run() {
    println!("init (dry run)")
}
