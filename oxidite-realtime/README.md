# oxidite-realtime

WebSocket and real-time features for Oxidite.

## Installation

```toml
[dependencies]
oxidite-realtime = "0.1"
```

## Usage

```rust
use oxidite_realtime::*;

// Create WebSocket manager
let ws_manager = WebSocketManager::new();

// Handle WebSocket connections
router.get("/ws", move |req| {
    ws_manager.handle(req)
});

// Broadcast to all clients
ws_manager.broadcast("room1", "Hello everyone!").await;

// Send to specific client
ws_manager.send_to(&client_id, "Direct message").await;

// Join room
ws_manager.join(&client_id, "room1").await;
```

## Features

- WebSocket support
- Room management
- Pub/sub messaging
- Direct messaging

## License

MIT
