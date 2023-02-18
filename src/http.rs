//! The http module is responsible for `ig`'s calls to the gitignore.io API.

use crate::error::{Error, Result};

/// Client for the gitignore.io API
pub struct Client<'a> {
    base_url: &'a str,
    client: reqwest::blocking::Client,
}

impl<'a> Client<'a> {
    /// Create a new client for the gitignore.io API
    pub fn new(base_url: &'a str) -> Self {
        Self {
            base_url,
            client: reqwest::blocking::Client::new(),
        }
    }

    /// Get the list of available targets from the gitignore.io API
    pub fn fetch_available_targets(&self) -> Result<String> {
        let response = self
            .client
            .get(format!("{}/list?format=lines", self.base_url))
            .send()?
            .error_for_status()?;

        let contents = response.text()?;
        Ok(contents)
    }

    /// Get the gitignore for the given targets from the gitignore.io API
    pub fn fetch_gitignore(&self, targets: &[&str]) -> Result<String> {
        let response = self
            .client
            .get(format!("{}/{}", self.base_url, targets.join(",")))
            .send()?
            .error_for_status();

        let response = match response {
            Ok(response) => response,
            Err(e) => {
                if e.status() == Some(reqwest::StatusCode::NOT_FOUND) {
                    return Err(Error::InvalidTarget(targets.join(",")));
                }
                return Err(Error::Http(e));
            }
        };

        let contents = response.text()?;
        Ok(contents)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    type TestResult = std::result::Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn test_fetch_available_targets_ok() -> TestResult {
        let mut server = mockito::Server::new();
        let url = &server.url();
        let client = Client::new(url);
        let body = "target1\ntarget2\ntarget3\n";

        let _mock = server
            .mock("GET", "/list?format=lines")
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body(body)
            .create();

        let response = client.fetch_available_targets()?;
        assert_eq!(response, body);
        Ok(())
    }

    #[test]
    fn test_fetch_available_targets_bad_request() {
        let mut server = mockito::Server::new();
        let url = &server.url();
        let client = Client::new(url);

        let _mock = server
            .mock("GET", "/list?format=lines")
            .with_status(400)
            .create();

        let response = client.fetch_available_targets();
        assert!(response.is_err());
    }

    #[test]
    fn test_fetch_available_targets_server_error() {
        let mut server = mockito::Server::new();
        let url = &server.url();
        let client = Client::new(url);

        let _mock = server
            .mock("GET", "/list?format=lines")
            .with_status(500)
            .create();

        let response = client.fetch_available_targets();
        assert!(response.is_err());
    }

    #[test]
    fn test_fetch_gitignore_ok() -> TestResult {
        let mut server = mockito::Server::new();
        let url = &server.url();
        let client = Client::new(url);
        let body = "target1\ntarget2\ntarget3\n";

        let _mock = server
            .mock("GET", "/target1,target2,target3")
            .with_status(200)
            .with_header("content-type", "text/plain")
            .with_body(body)
            .create();

        let response = client.fetch_gitignore(&["target1", "target2", "target3"])?;
        assert_eq!(response, body);
        Ok(())
    }

    #[test]
    fn test_fetch_gitignore_bad_request() {
        let mut server = mockito::Server::new();
        let url = &server.url();
        let client = Client::new(url);

        let _mock = server
            .mock("GET", "/target1,target2,target3")
            .with_status(400)
            .create();

        let response = client.fetch_gitignore(&["target1", "target2", "target3"]);
        assert!(response.is_err());
    }

    #[test]
    fn test_fetch_gitignore_server_error() {
        let mut server = mockito::Server::new();
        let url = &server.url();
        let client = Client::new(url);

        let _mock = server
            .mock("GET", "/target1,target2,target3")
            .with_status(500)
            .create();

        let response = client.fetch_gitignore(&["target1", "target2", "target3"]);
        assert!(response.is_err());
    }
}
