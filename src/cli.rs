//! The cli module defines the CLI argument parsing for the `ig` binary

use clap::Parser;

const LONG_ABOUT: &str = "
Generate great gitignore files, straight from the command line! 🛠️
";

#[derive(Parser, Debug)]
#[command(author, version, about, long_about=LONG_ABOUT)]
struct Cli {
    /// List of targets to generate a gitignore for
    targets: Vec<String>,

    /// Print the list of available targets
    #[arg(short, long)]
    list: bool,
}

/// Parse the CLI arguments and run the program
pub fn run() {
    let cli = Cli::parse();
    dbg!(&cli);

    if cli.list {
        println!("Listing available targets...");
        return;
    }

    println!("Generating gitignore for {:?}", cli.targets);
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
