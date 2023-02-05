//! The cli module defines the CLI argument parsing for the `ig` binary

use std::error::Error;

use clap::Parser;

use crate::http;

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
    #[arg(short, long)]
    stdout: bool,
}

/// Parse the CLI arguments and run the program
pub fn run() -> Result<(), Box<dyn Error>> {
    let cli = Cli::parse();
    dbg!(&cli);

    let client = http::Client::new(BASE_URL);

    if cli.list {
        println!("Listing available targets...");
        let targets = client.fetch_available_targets()?;
        println!("{targets}");
        return Ok(());
    }

    println!("Generating gitignore for {:?}", cli.targets);

    if cli.stdout {
        println!("Printing to stdout...");
    } else {
        println!("Writing to file...");
    }

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
