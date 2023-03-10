//! The cli module defines the CLI argument parsing for the `ig` binary

use crate::{error::Error, error::Result, http};
use clap::Parser;
use colored::Colorize;

const LONG_ABOUT: &str = "
Generate great gitignore files, straight from the command line! 🛠️
";

const BASE_URL: &str = "https://www.toptal.com/developers/gitignore/api";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=LONG_ABOUT)]
struct Cli {
    /// List of targets to generate a gitignore for
    targets: Vec<String>,

    /// Print the list of available targets
    #[arg(short, long)]
    list: bool,

    /// Print the gitignore to stdout instead of writing to a file
    #[arg(short, long, conflicts_with = "force")]
    stdout: bool,

    /// Force overwrite of an existing .gitignore file
    #[arg(short, long)]
    force: bool,
}

/// Parse the CLI arguments and run the program
pub fn run() -> Result<()> {
    let cli = Cli::parse();

    let client = http::Client::new(BASE_URL);

    if cli.list {
        let targets = client.fetch_available_targets()?;
        println!("{targets}");
        return Ok(());
    }

    if cli.targets.is_empty() {
        return Err(Error::NoTargets);
    }

    let targets: Vec<&str> = cli
        .targets
        .iter()
        .map(std::string::String::as_str)
        .collect();
    let gitignore = client.fetch_gitignore(&targets)?;

    if cli.stdout {
        println!("{gitignore}");
    } else {
        let ignore_file = std::env::current_dir()?.join(".gitignore");
        if ignore_file.exists() && !cli.force {
            return Err(Error::FileAlreadyExists { cwd: ignore_file });
        }
        std::fs::write(ignore_file, gitignore)?;
    }

    println!(
        "{}: {}",
        "Success".green().bold(),
        "gitignore written!".bold().white(),
    );

    Ok(())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn cli_smoke_test() {
        use clap::CommandFactory;
        Cli::command().debug_assert();
    }
}
