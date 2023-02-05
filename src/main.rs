#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

use std::error::Error;

mod cli;
mod http;

fn main() -> Result<(), Box<dyn Error>> {
    cli::run()?;
    Ok(())
}
