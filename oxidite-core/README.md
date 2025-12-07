# oxidite-core

Core HTTP server and routing for the Oxidite web framework.

## Installation

```toml
[dependencies]
oxidite-core = "0.1"
tokio = { version = "1", features = ["full"] }
```

## Usage

### Basic Server

```rust
use oxidite_core::*;

#[tokio::main]
async fn main() -> Result<()> {
    let mut router = Router::new();
    
    router.get("/", |_req| async {
        Ok(Response::text("Hello!"))
    });
    
    Server::new(router)
        .listen("127.0.0.1:3000".parse().unwrap())
        .await
}
```

### Path Parameters

```rust
use oxidite_core::extract::Path;
use std::collections::HashMap;

router.get("/users/:id", |Path(params): Path<HashMap<String, String>>| async move {
    let id = params.get("id").unwrap();
    Ok(Response::text(format!("User {}", id)))
});
```

### JSON Responses

```rust
use oxidite_core::extract::Json;
use serde::{Serialize, Deserialize};

#[derive(Serialize)]
struct User {
    id: i64,
    name: String,
}

router.get("/users", |_req| async {
    Ok(Json(User { id: 1, name: "John".into() }))
});
```

## Features

- HTTP/1.1 and HTTP/2 support
- WebSocket support
- Path parameter extraction
- Query parameter parsing
- JSON request/response
- Cookie parsing
- Form data handling
- Type-safe extractors

## License

MIT
