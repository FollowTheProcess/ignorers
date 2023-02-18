#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod cli;
mod error;
mod http;

fn main() {
    if let Err(e) = cli::run() {
        eprintln!("Error: {e}");
        std::process::exit(1);
    }
}
