#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod cli;
mod error;
mod http;

use colored::Colorize;

fn main() {
    if let Err(e) = cli::run() {
        eprintln!("{}: {}", "Error".red().bold(), e.to_string().bold().white());
        std::process::exit(1);
    }
}
