use http::{Method, HeaderMap, HeaderName, HeaderValue};
use http_body_util::Full;
use bytes::Bytes;
use serde::Serialize;

/// Test request builder
pub struct TestRequest {
    method: Method,
    uri: String,
    headers: HeaderMap,
    body: Vec<u8>,
}

impl TestRequest {
    /// Create a new test request builder
    pub fn new(method: Method, uri: impl Into<String>) -> Self {
        Self {
            method,
            uri: uri.into(),
            headers: HeaderMap::new(),
            body: Vec::new(),
        }
    }

    /// Create a GET request
    pub fn get(uri: impl Into<String>) -> Self {
        Self::new(Method::GET, uri)
    }

    /// Create a POST request
    pub fn post(uri: impl Into<String>) -> Self {
        Self::new(Method::POST, uri)
    }

    /// Create a PUT request
    pub fn put(uri: impl Into<String>) -> Self {
        Self::new(Method::PUT, uri)
    }

    /// Create a DELETE request
    pub fn delete(uri: impl Into<String>) -> Self {
        Self::new(Method::DELETE, uri)
    }

    /// Add a header
    pub fn header(mut self, name: impl Into<String>, value: impl Into<String>) -> Self {
        let name = HeaderName::from_bytes(name.into().as_bytes()).unwrap();
        let value = HeaderValue::from_str(&value.into()).unwrap();
        self.headers.insert(name, value);
        self
    }

    /// Set JSON body
    pub fn json<T: Serialize>(mut self, body: &T) -> Self {
        self.body = serde_json::to_vec(body).unwrap();
        self = self.header("content-type", "application/json");
        self
    }

    /// Set raw body
    pub fn body(mut self, body: impl Into<Vec<u8>>) -> Self {
        self.body = body.into();
        self
    }

    /// Build the request
    pub fn build(self) -> http::Request<Full<Bytes>> {
        let mut builder = http::Request::builder()
            .method(self.method)
            .uri(self.uri);

        for (name, value) in self.headers.iter() {
            builder = builder.header(name, value);
        }

        builder.body(Full::new(Bytes::from(self.body))).unwrap()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use serde::Deserialize;

    #[derive(Serialize, Deserialize)]
    struct TestData {
        name: String,
    }

    #[test]
    fn test_request_builder() {
        let data = TestData {
            name: "test".to_string(),
        };

        let request = TestRequest::post("/api/test")
            .json(&data)
            .header("x-custom", "value")
            .build();

        assert_eq!(request.method(), Method::POST);
        assert_eq!(request.uri(), "/api/test");
        assert!(request.headers().contains_key("content-type"));
    }
}
