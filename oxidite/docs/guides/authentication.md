# Authentication Guide

Complete guide to implementing authentication in Oxidite applications.

## Installation

```toml
[dependencies]
oxidite = { version = "1.0", features = ["auth", "database"] }
```

## Quick Start

```rust
use oxidite::prelude::*;
use oxidite::auth::*;

#[tokio::main]
async fn main() -> Result<()> {
    let mut app = Router::new();
    
    app.post("/login", login);
    app.get("/profile", profile).middleware(AuthMiddleware);
    
    Server::new(app).listen("127.0.0.1:3000".parse()?).await
}

async fn login(Json(creds): Json<LoginRequest>) -> Result<Json<TokenResponse>> {
    // Verify credentials
    let user = User::find_by_email(&creds.email).await?;
    
    if !verify_password(&creds.password, &user.password_hash)? {
        return Err(Error::Unauthorized);
    }
    
    // Create JWT
    let token = create_jwt(&user, "your-secret-key")?;
    
    Ok(Json(TokenResponse { token }))
}

async fn profile(auth: Auth) -> Result<Json<User>> {
    Ok(Json(auth.user))
}
```

## Password Hashing

```rust
use oxidite::auth::password::*;

// Hash password (Argon2)
let hash = hash_password("password123")?;

// Verify password
let valid = verify_password("password123", &hash)?;
```

## JWT Authentication

```rust
use oxidite::auth::jwt::*;

// Create token
let token = create_jwt(&user, "secret-key")?;

// Verify token
let claims = verify_jwt(&token, "secret-key")?;
let user_id = claims.sub;
```

## RBAC (Roles & Permissions)

```rust
use oxidite::auth::rbac::*;

// Assign role
user.assign_role("admin").await?;

// Check permission
if user.has_permission("users.delete") {
    // Allow
}

// Middleware
app.delete("/users/:id", delete_user)
    .middleware(RequirePermission::new("users.delete"));
```

## API Keys

```rust
use oxidite::auth::api_key::*;

// Generate API key
let key = ApiKey::generate(user_id)?;

// Validate
if ApiKey::validate(&key_string, &hash).await? {
    // Valid
}
```

## 2FA (Two-Factor Authentication)

```rust
use oxidite::auth::totp::*;

// Setup
let secret = generate_totp_secret();
let qr_code = generate_qr_code(&secret, "user@example.com")?;

// Verify  
let valid = verify_totp(&secret, &user_code)?;
```

Complete examples at [docs.rs/oxidite](https://docs.rs/oxidite)
