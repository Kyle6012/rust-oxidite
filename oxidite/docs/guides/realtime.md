# Real-time & WebSocket Guide

Build real-time applications with WebSockets in Oxidite.

## Installation

```toml
[dependencies]
oxidite = { version = "1.0", features = ["realtime"] }
```

## Quick Start

```rust
use oxidite::prelude::*;
use oxidite::realtime::*;

#[tokio::main]
async fn main() -> Result<()> {
    let ws_manager = WebSocketManager::new();
    
    let mut app = Router::new();
    
    // WebSocket endpoint
    app.get("/ws", {
        let manager = ws_manager.clone();
        move |req| {
            let manager = manager.clone();
            async move {
                manager.handle(req).await
            }
        }
    });
    
    Server::new(app).listen("127.0.0.1:3000".parse()?).await
}
```

## Broadcasting

```rust
// Broadcast to all clients
ws_manager.broadcast("Hello everyone!").await;

// Broadcast to room
ws_manager.broadcast_to_room("room1", "Room message").await;
```

## Direct Messaging

```rust
// Send to specific client
ws_manager.send_to(&client_id, "Direct message").await;
```

## Room Management

```rust
// Join room
ws_manager.join(&client_id, "chat-room").await;

// Leave room
ws_manager.leave(&client_id, "chat-room").await;

// List clients in room
let clients = ws_manager.room_clients("chat-room").await;
```

## Complete Chat Example

```rust
use oxidite::prelude::*;
use oxidite::realtime::*;
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize)]
struct ChatMessage {
    user: String,
    message: String,
    room: String,
}

#[tokio::main]
async fn main() -> Result<()> {
    let ws_manager = Arc::new(WebSocketManager::new());
    
    let mut app = Router::new();
    
    // WebSocket connection
    app.get("/ws", {
        let manager = ws_manager.clone();
        move |req| {
            let manager = manager.clone();
            async move {
                manager.handle(req).await
            }
        }
    });
    
    // Send message to room
    app.post("/chat/send", {
        let manager = ws_manager.clone();
        move |Json(msg): Json<ChatMessage>| {
            let manager = manager.clone();
            async move {
                manager.broadcast_to_room(&msg.room, &msg).await;
                Ok(Json(json!({ "status": "sent " })))
            }
        }
    });
    
    Server::new(app).listen("127.0.0.1:3000".parse()?).await
}
```

## Client (JavaScript)

```javascript
const ws = new WebSocket('ws://localhost:3000/ws');

ws.onopen = () => {
    console.log('Connected');
    ws.send(JSON.stringify({
        type: 'join',
        room: 'general'
    }));
};

ws.onmessage = (event) => {
    const data = JSON.parse(event.data);
    console.log('Received:', data);
};

// Send message
function sendMessage(message) {
    ws.send(JSON.stringify({
        type: 'message',
        content: message
    }));
}
```

## Advanced Features

### Presence Tracking

```rust
// Track online users
ws_manager.on_connect(|client_id| {
    println!("User {} connected", client_id);
});

ws_manager.on_disconnect(|client_id| {
    println!("User {} disconnected", client_id);
});
```

### Message Persistence

```rust
use oxidite::queue::*;

// Save messages to database
#[derive(Serialize, Deserialize)]
struct SaveMessageJob {
    message: ChatMessage,
}

#[async_trait]
impl Job for SaveMessageJob {
    async fn perform(&self) -> JobResult {
        // Save to database
        Message::create(&db, &self.message).await?;
        Ok(())
    }
}
```

Complete examples at [docs.rs/oxidite](https://docs.rs/oxidite)
