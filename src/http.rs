//! The http module is responsible for `ig`'s calls to the gitignore.io API.

/// Client for the gitignore.io API
pub struct Client {
    base_url: String,
    client: reqwest::blocking::Client,
}

impl Client {
    /// Create a new client for the gitignore.io API
    pub fn new(base_url: &str) -> Self {
        Self {
            base_url: base_url.to_string(),
            client: reqwest::blocking::Client::new(),
        }
    }

    /// Get the list of available targets from the gitignore.io API
    pub fn fetch_available_targets(&self) -> Result<String, reqwest::Error> {
        let response = self
            .client
            .get(format!("{}/list?format=lines", self.base_url))
            .send()?
            .error_for_status()?;

        let contents = response.text()?;
        Ok(contents)
    }
}

#[cfg(test)]
mod test {
    use super::*;

    type TestResult = Result<(), Box<dyn std::error::Error>>;

    #[test]
    fn test_fetch_available_targets_ok() -> TestResult {
        let url = &mockito::server_url();
        let client = Client::new(url);
        let body = "target1\ntarget2\ntarget3\n";

        let _mock = mockito::mock("GET", "/list?format=lines")
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
        let url = &mockito::server_url();
        let client = Client::new(url);

        let _mock = mockito::mock("GET", "/list?format=lines")
            .with_status(400)
            .create();

        let response = client.fetch_available_targets();
        assert!(response.is_err());
    }

    #[test]
    fn test_fetch_available_targets_server_error() {
        let url = &mockito::server_url();
        let client = Client::new(url);

        let _mock = mockito::mock("GET", "/list?format=lines")
            .with_status(500)
            .create();

        let response = client.fetch_available_targets();
        assert!(response.is_err());
    }
}
