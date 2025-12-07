# oxidite-auth

Authentication and authorization for Oxidite (RBAC, JWT, OAuth2, 2FA, API keys).

## Installation

```toml
[dependencies]
oxidite-auth = "0.1"
```

## Usage

### JWT Authentication

```rust
use oxidite_auth::*;

// Create JWT
let token = create_jwt(&user, "your-secret-key")?;

// Verify JWT
let claims = verify_jwt(&token, "your-secret-key")?;
```

### Password Hashing

```rust
use oxidite_auth::password::*;

// Hash password
let hash = hash_password("password123")?;

// Verify password
let valid = verify_password("password123", &hash)?;
```

### RBAC (Role-Based Access Control)

```rust
use oxidite_auth::rbac::*;

// Create role
let admin = Role {
    name: "admin".to_string(),
    permissions: vec!["users.create", "users.delete"],
};

// Check permission
if user.has_permission("users.delete") {
    // Allow action
}
```

### API Key Authentication

```rust
use oxidite_auth::api_key::*;

// Generate API key
let key = ApiKey::generate(user_id)?;

// Validate
if ApiKey::validate(&key_string, &hash).await? {
    // Valid key
}
```

### 2FA (Two-Factor Authentication)

```rust
use oxidite_auth::totp::*;

// Generate secret
let secret = generate_totp_secret();

// Verify code  
let valid = verify_totp(&secret, &user_code)?;
```

## Features

- JWT token generation/verification
- Password hashing (Argon2)
- RBAC with roles and permissions
- API key authentication
- Two-Factor Authentication (TOTP)
- OAuth2 integration
- Email verification tokens
- Password reset tokens

## License

MIT
