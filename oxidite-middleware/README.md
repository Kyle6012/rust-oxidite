# oxidite-middleware

HTTP middleware for Oxidite (CORS, logging, compression, rate limiting).

## Installation

```toml
[dependencies]
oxidite-middleware = "0.1"
```

## Usage

### CORS

```rust
use oxidite_middleware::*;

let cors = CorsLayer::new()
    .allow_origin("https://example.com")
    .allow_methods(vec!["GET", "POST"])
    .allow_headers(vec!["Content-Type"]);

let app = ServiceBuilder::new()
    .layer(cors)
    .service(router);
```

### Logging

```rust
let app = ServiceBuilder::new()
    .layer(LoggerLayer)
    .service(router);
```

### Compression

```rust
let app = ServiceBuilder::new()
    .layer(CompressionLayer::new())
    .service(router);
```

### Rate Limiting

```rust
let limiter = RateLimitLayer::new(100, Duration::from_secs(60)); // 100 req/min

let app = ServiceBuilder::new()
    .layer(limiter)
    .service(router);
```

### Multiple Middleware

```rust
let app = ServiceBuilder::new()
    .layer(LoggerLayer)
    .layer(CorsLayer::permissive())
    .layer(CompressionLayer::new())
    .layer(RateLimitLayer::default())
    .service(router);
```

## Available Middleware

- **CorsLayer** - Cross-Origin Resource Sharing
- **LoggerLayer** - Request/response logging
- **CompressionLayer** - Gzip compression
- **RateLimitLayer** - Rate limiting
- **AuthMiddleware** - Authentication
- **CsrfLayer** - CSRF protection

## License

MIT
