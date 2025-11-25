# CypherTrade Service Bus Contracts

A Rust library that defines service bus contracts and event models for the CypherTrade service bus system. This library provides type-safe, serializable event structures using Protocol Buffers (protobuf) and Serde.

## Overview

This crate contains the shared contract definitions for events and messages that flow through the CypherTrade service bus. It ensures type safety and consistency across services that publish and consume these events.

## Features

- **Type-safe event models** using Protocol Buffers (prost)
- **JSON serialization** support via Serde
- **Service bus integration** with my-service-bus SDK
- **Structured event definitions** for domain events

## Project Structure

```
src/
├── lib.rs              # Library root, exports modules
└── user/
    ├── mod.rs          # User module
    └── user_registered.rs  # UserRegistered event definition
```

## Current Events

### User Events

- **`UserRegisteredSbEvent`**: Published when a new user registers
  - Contains user information (id, email)
  - Includes source tracking metadata

## Dependencies

- **yft-service-sdk**: Service bus SDK with my-service-bus integration
- **prost**: Protocol Buffers implementation for Rust
- **serde**: Serialization framework

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
cyphertrade-service-bus-contracts = { path = "../cyphertrade-service-bus-contracts" }
```

### Example: Publishing an Event

```rust
use cyphertrade_service_bus_contracts::user::user_registered::{
    UserRegisteredSbEvent, UserSbModel
};
use std::collections::HashMap;

let event = UserRegisteredSbEvent {
    user: Some(UserSbModel {
        id: "user-123".to_string(),
        email: "user@example.com".to_string(),
    }),
    sources: HashMap::new(),
};

// Publish to service bus using your service bus client
```

### Example: Consuming an Event

```rust
use cyphertrade_service_bus_contracts::user::user_registered::UserRegisteredSbEvent;

// In your service bus subscriber handler
fn handle_user_registered(event: UserRegisteredSbEvent) {
    if let Some(user) = event.user {
        println!("User registered: {} ({})", user.email, user.id);
    }
}
```

## Development

### Prerequisites

- Rust toolchain (latest stable version)
- Cargo

### Setup

1. Clone the repository
2. Install pre-commit hooks (optional but recommended):
   ```bash
   pre-commit install
   pre-commit install --hook-type pre-push
   ```

### Code Quality

This project uses pre-commit hooks to ensure code quality:

- **`cargo fmt`**: Formats code according to Rust style guidelines
- **`cargo clippy`**: Lints code for common mistakes and improvements
- **`cargo check`**: Verifies the project compiles
- **`cargo test`**: Runs tests (on pre-push)

Run these manually:
```bash
cargo fmt
cargo clippy
cargo test
```

### Adding New Events

1. Create a new module in the appropriate domain directory (e.g., `src/user/`)
2. Define your event struct with:
   - `#[derive(Clone, PartialEq, ::prost::Message, Serialize, Deserialize)]`
   - `#[my_sb_entity_protobuf_model(topic_id = "your-topic-id")]`
3. Export the module in the parent `mod.rs`
4. Update this README with the new event documentation

## Versioning

This project follows semantic versioning. Breaking changes to event structures will result in a major version bump.

## License

[Add your license here]

