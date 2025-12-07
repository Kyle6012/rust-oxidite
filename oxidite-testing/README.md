# Oxidite Testing

Testing utilities for the Oxidite web framework.

## Features

- **Test Request Builder**: Easily create HTTP requests for testing
- **Test Response Helpers**: Parse and assert on responses
- **Async Test Support**: Built-in async test utilities

## Usage

Add to your `Cargo.toml`:

```toml
[dev-dependencies]
oxidite-testing = "0.1"
```

## Example

```rust
use oxidite_testing::*;

#[tokio::test]
async fn test_api_endpoint() {
    let request = TestRequest::get("/api/users")
        .header("authorization", "Bearer token")
        .build();
    
    // Test your handler
    let response = my_handler(request).await.unwrap();
    assert!(response.is_success());
}

#[tokio::test]
async fn test_post_json() {
    #[derive(Serialize)]
    struct CreateUser {
        name: String,
        email: String,
    }
    
    let user = CreateUser {
        name: "Test".to_string(),
        email: "test@example.com".to_string(),
    };
    
    let request = TestRequest::post("/api/users")
        .json(&user)
        .build();
    
    // Test your handler
}
```

## License

MIT
