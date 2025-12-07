# oxidite-db

Database ORM with relationships and migrations for Oxidite.

## Installation

```toml
[dependencies]
oxidite-db = "0.1"
tokio = { version = "1", features = ["full"] }
serde = { version = "1", features = ["derive"] }
```

## Usage

### Define Models

```rust
use oxidite_db::*;
use serde::{Serialize, Deserialize};

#[derive(Model, Serialize, Deserialize)]
#[table_name = "users"]
struct User {
    pub id: i64,
    pub name: String,
    pub email: String,
    pub created_at: chrono::DateTime<chrono::Utc>,
}
```

### Query Data

```rust
// Find all
let users = User::all().await?;

// Find by ID
let user = User::find(1).await?;

// Where clause
let user = User::where_eq("email", "john@example.com").first().await?;

// Create
let user = User {
    name: "John".to_string(),
    email: "john@example.com".to_string(),
    ..Default::default()
};
user.save().await?;

// Update
user.name = "Jane".to_string();
user.save().await?;

// Delete
user.delete().await?;
```

### Relationships

```rust
#[derive(Model)]
struct Post {
    pub id: i64,
    pub user_id: i64,
    pub title: String,
}

// Load relationship
let posts = user.has_many::<Post>("user_id").await?;
```

### Migrations

```rust
use oxidite_db::Migration;

struct CreateUsers;

impl Migration for CreateUsers {
    fn up(&self) -> String {
        "CREATE TABLE users (
            id SERIAL PRIMARY KEY,
            name VARCHAR NOT NULL,
            email VARCHAR UNIQUE NOT NULL,
            created_at TIMESTAMP DEFAULT NOW()
        )".to_string()
    }
    
    fn down(&self) -> String {
        "DROP TABLE users".to_string()
    }
}
```

## Features

- Model derive macro
- Relationships (HasOne, HasMany, BelongsTo)
- Migrations with rollback
- Soft deletes
- Timestamps
- Field validation
- Transaction support

## License

MIT
