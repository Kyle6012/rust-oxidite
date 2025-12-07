use serde::de::DeserializeOwned;
use http::StatusCode;

/// Test response wrapper
pub struct TestResponse {
    status: StatusCode,
    body: Vec<u8>,
}

impl TestResponse {
    /// Create a new test response
    pub fn new(status: StatusCode, body: Vec<u8>) -> Self {
        Self { status, body }
    }

    /// Get status code
    pub fn status(&self) -> StatusCode {
        self.status
    }

    /// Get body as bytes
    pub fn body(&self) -> &[u8] {
        &self.body
    }

    /// Get body as string
    pub fn text(&self) -> Result<String, std::string::FromUtf8Error> {
        String::from_utf8(self.body.clone())
    }

    /// Deserialize JSON body
    pub fn json<T: DeserializeOwned>(&self) -> Result<T, serde_json::Error> {
        serde_json::from_slice(&self.body)
    }

    /// Check if response is successful (2xx)
    pub fn is_success(&self) -> bool {
        self.status.is_success()
    }
}
