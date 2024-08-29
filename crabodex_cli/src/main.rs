use std::io::{self, Write};
use std::path::PathBuf;
use clap::Parser;

const DEFAULT_IGNORE_FOLDERS: &[&str] = &[
    ".git/",
    ".svn/",
    ".hg/",
    "build/",
    "dist/",
    "out/",
    "bin/",
    "target/",
    ".idea/",
    ".vscode/",
    ".vs/",
    ".eclipse/",
    "node_modules/",
];

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

    #[clap(long, default_value = "")]
    repo_url: String,

    #[clap(long, use_value_delimiter = true, value_delimiter = ',')]
    ignore_folders: Vec<String>,
}

fn main() -> io::Result<()> {
    let cli: Cli = Cli::parse();

    let mut merged_ignore_folders: Vec<String> = DEFAULT_IGNORE_FOLDERS
        .iter()
        .map(|&s| s.to_string())
        .collect();
    merged_ignore_folders.extend(cli.ignore_folders);
    merged_ignore_folders.sort_unstable();
    merged_ignore_folders.dedup();

    let result: String = crabodex_lib::generate(
        &cli.root_directory,
        &cli.repo_name,
        &cli.repo_description,
        &cli.commit_hash,
        &cli.repo_url,
        &merged_ignore_folders
    );

    io::stdout().write_all(result.as_bytes())?;
    Ok(())
}
