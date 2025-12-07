# oxidite-cache

Caching backends (Memory, Redis) for the Oxidite web framework.

## Installation

```toml
[dependencies]
oxidite-cache = "0.1"
```

## Usage

### Memory Cache

```rust
use oxidite_cache::*;

let cache = MemoryCache::new();

// Set value
cache.set("key", "value", Some(Duration::from_secs(3600))).await?;

// Get value
if let Some(value) = cache.get::<String>("key").await? {
    println!("Value: {}", value);
}

// Delete
cache.delete("key").await?;
```

### Redis Cache

```rust
let cache = RedisCache::new("redis://127.0.0.1")?;

cache.set("key", "value", Some(Duration::from_secs(3600))).await?;
```

### Remember Pattern

```rust
let value = cache.remember("expensive-key", Duration::from_secs(3600), || async {
    // Expensive computation
    calculate_something().await
}).await?;
```

### Tagged Cache

```rust
// Set with tags
cache.set_tagged("user:1", data, vec!["users"], None).await?;

// Invalidate by tag
cache.invalidate_tag("users").await?;
```

## Features

- Memory backend (LRU)
- Redis backend
- TTL support
- Remember pattern
- Tagged cache invalidation
- Async/await

## License

MIT
