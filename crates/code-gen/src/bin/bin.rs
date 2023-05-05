use clap::{CommandFactory, Parser};
use miette::{IntoDiagnostic, Result};
use std::fs::File;
use std::io::prelude::*;
use std::path::{Path, PathBuf};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
struct Cli {
    #[arg(short, long, value_name = "FILE")]
    file: Option<PathBuf>,
}

fn run_codegen(file_path: &Path) -> Result<String> {
    let mut file = File::open(file_path).into_diagnostic()?;
    let mut contents = String::new();
    file.read_to_string(&mut contents).into_diagnostic()?;
    code_gen::gen(contents).into_diagnostic()
}

fn main() -> Result<()> {
    let cli = Cli::parse();

    let file_path = cli.file.as_deref().unwrap_or_else(|| {
        Cli::command()
            .error(
                clap::error::ErrorKind::MissingRequiredArgument,
                "Missing input file",
            )
            .exit();
    });

    run_codegen(file_path)?;
    Ok(())
}
