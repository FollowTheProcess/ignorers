//! The error crate is responsible for defining the error types used in the application. It also provides the `Result` type alias, which is used throughout the application.

/// The result type used throughout the application
pub type Result<T> = std::result::Result<T, Error>;

#[derive(thiserror::Error, Debug)]
pub enum Error {
    /// The requested target is not valid
    #[error("Invalid target(s): {0}, see available targets with `ig --list`")]
    InvalidTarget(String),

    /// Another error occurred while making a request to the API
    #[error("HTTP error: {0}")]
    Http(#[from] reqwest::Error),

    /// A gitignore file already exists in the current directory
    #[error("A .gitignore file already exists at {cwd:?}, use `--force` to overwrite")]
    FileAlreadyExists { cwd: std::path::PathBuf },

    /// An error occurred while reading or writing to a file
    #[error("Error reading or writing from file {0}")]
    ReadWrite(#[from] std::io::Error),
}
