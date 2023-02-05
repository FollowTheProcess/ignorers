#![warn(clippy::pedantic)]
#![forbid(unsafe_code)]
#![doc = include_str!("../README.md")]

mod cli;

fn main() {
    cli::run();
}
