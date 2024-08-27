use std::io::{self, Write};
use std::path::PathBuf;
use clap::Parser;
use crabodex_lib;

#[derive(Parser)]
#[clap(author, version, about, long_about = None)]
struct Cli {
    #[clap(long, default_value = ".")]
    root_directory: PathBuf,

    #[clap(long, default_value = "Documentation")]
    repo_name: String,

    #[clap(long, default_value = "")]
    repo_description: String,

    #[clap(long, default_value = "latest")]
    commit_hash: String,

    #[clap(long, default_value = "latest")]
    repo_url: String,

    #[clap(long, use_value_delimiter = true, value_delimiter = ',')]
    ignore_folders: Vec<String>,
}

fn main() -> io::Result<()> {
    let cli: Cli = Cli::parse();

    let ignore_folders: Vec<&str> = cli.ignore_folders.iter().map(AsRef::as_ref).collect();

    let result: String = crabodex_lib::generate(
        &cli.root_directory,
        &cli.repo_name,
        &cli.repo_description,
        &cli.commit_hash,
        &cli.repo_url,
        if ignore_folders.is_empty() { None } else { Some(&ignore_folders) }
    );

    io::stdout().write_all(result.as_bytes())?;
    Ok(())
}
