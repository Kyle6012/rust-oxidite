use crate::error::{Error, Result};
use crate::extract::FromRequest;
use crate::types::OxiditeRequest;

/// Cookie extractor for typed cookie access
///
/// # Example
/// ```ignore
/// use oxidite_core::{Cookies, Response};
///
/// async fn handler(cookies: Cookies) -> Result<Response> {
///     if let Some(value) = cookies.get("session_id") {
///         // Use cookie value
///     }
///     Ok(Response::ok())
/// }
/// ```
pub struct Cookies {
    cookies: Vec<(String, String)>,
}

impl Cookies {
    /// Get cookie value by name
    pub fn get(&self, name: &str) -> Option<&str> {
        self.cookies
            .iter()
            .find(|(k, _)| k == name)
            .map(|(_, v)| v.as_str())
    }
    
    /// Get all cookies
    pub fn iter(&self) -> impl Iterator<Item = (&str, &str)> {
        self.cookies.iter().map(|(k, v)| (k.as_str(), v.as_str()))
    }
}

/// FromRequest implementation for Cookies
impl FromRequest for Cookies {
    async fn from_request(req: &mut OxiditeRequest) -> Result<Self> {
        let cookie_header = req
            .headers()
            .get("cookie")
            .and_then(|h| h.to_str().ok())
            .unwrap_or("");
            
        let mut cookies = Vec::new();
        for cookie_str in cookie_header.split(';') {
            let trimmed = cookie_str.trim();
            if let Some((name, value)) = trimmed.split_once('=') {
                cookies.push((name.to_string(), value.to_string()));
            }
        }
        
        Ok(Cookies { cookies })
    }
}

/// Form data extractor for application/x-www-form-urlencoded
///
/// # Example
/// ```ignore
/// use serde::Deserialize;
///
/// #[derive(Deserialize)]
/// struct LoginForm {
///     username: String,
///     password: String,
/// }
///
/// async fn login(form: Form<LoginForm>) -> Result<Response> {
///     // Use form.0 to access data
///     Ok(Response::ok())
/// }
/// ```
pub struct Form<T>(pub T);

impl<T: serde::de::DeserializeOwned> FromRequest for Form<T> {
    async fn from_request(req: &mut OxiditeRequest) -> Result<Self> {
        use http_body_util::BodyExt;
        use bytes::Buf;

        let body = req.body_mut();
        let bytes = body.collect().await
            .map_err(|e| Error::Server(format!("Failed to read body: {}", e)))?
            .aggregate();
            
        let mut data = String::new();
        std::io::Read::read_to_string(&mut bytes.reader(), &mut data)
            .map_err(|e| Error::BadRequest(format!("Failed to read body: {}", e)))?;
        
        serde_urlencoded::from_str(&data)
            .map(Form)
            .map_err(|e| Error::BadRequest(format!("Invalid form data: {}", e)))
    }
}
