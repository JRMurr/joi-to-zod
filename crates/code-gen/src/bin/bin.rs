use clap::{CommandFactory, Parser};
use std::path::{Path, PathBuf};
use std::fs::File;
use code_gen::gen_from_file;
use std::io::prelude::*;


#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,
}

fn run_codegen(file_path: &Path) -> std::io::Result<()> {
    let mut file = File::open(file_path)?;
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    gen_from_file(contents);
    Ok(())
}

fn main() -> std::io::Result<()> {
    let cli = Cli::parse();

    let file_path = cli.file.as_deref().unwrap_or_else(|| {
        Cli::command()
            .error(
                clap::error::ErrorKind::MissingRequiredArgument,
                "Missing input file",
            )
            .exit();
    });

    run_codegen(file_path)
}
