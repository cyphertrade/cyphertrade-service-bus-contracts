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
├── lib.rs
├── battle/
│   ├── mod.rs
│   ├── battle_participation_update.rs
│   └── battle_registration_update.rs
├── contest/
│   ├── mod.rs
│   ├── contest_participation_update.rs
│   ├── contest_registration_update.rs
│   └── contest_update.rs
├── profile/
│   ├── mod.rs
│   ├── personal_data_updated.rs
│   └── public_profile_update.rs
└── user/
    ├── mod.rs
    ├── user_login.rs
    ├── user_registered.rs
    └── user_updated.rs
```

## Events

### User Events

| Event | Topic ID | Description |
|-------|----------|-------------|
| `UserRegisteredSbEvent` | `user-registered` | Published when a new user registers |
| `UserLoginSbEvent` | `user-login` | Published when a user logs in |
| `UserUpdatedSbEvent` | `user-updated` | Published when user data is updated |

### Profile Events

| Event | Topic ID | Description |
|-------|----------|-------------|
| `PersonalDataUpdatedSbEvent` | `personal-data-updated` | Published when user's personal data is updated |
| `PublicProfileUpdateSbEvent` | `public-profile-update` | Published when user's public profile is updated |

### Contest Events

| Event | Topic ID | Description |
|-------|----------|-------------|
| `ContestRegistrationUpdateSbEvent` | `contest-registration-update` | Published when user registers/cancels contest registration |
| `ContestParticipationUpdateSbEvent` | `contest-participation-update` | Published when participant status changes during contest |
| `ContestAccountsUpdateSbEvent` | `contest-accounts-updates` | Published for contest account updates |

### Battle Events

Battle is a special type of Contest with `min_participants = 2`, `max_participants = 2`, and `auto_start_when_full = true`.

| Event | Topic ID | Description |
|-------|----------|-------------|
| `BattleRegistrationUpdateSbEvent` | `battle-registration-update` | Published when user registers/cancels battle registration |
| `BattleParticipationUpdateSbEvent` | `battle-participation-update` | Published when participant status changes during battle |

## Dependencies

- **yft-service-sdk**: Service bus SDK with my-service-bus integration
- **prost**: Protocol Buffers implementation for Rust
- **serde**: Serialization framework

## Usage

Add this crate to your `Cargo.toml`:

```toml
[dependencies]
cyphertrade-service-bus-contracts = { git = "...", tag = "x.x.x" }
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

### Example: Consuming a Contest Event

```rust
use cyphertrade_service_bus_contracts::contest::contest_participation_update::ContestParticipationUpdateSbEvent;

fn handle_contest_participation(event: ContestParticipationUpdateSbEvent) {
    println!(
        "User {} participation in contest {}: status={}",
        event.user_id, event.contest_id, event.status
    );
    
    if let Some(rank) = event.rank {
        println!("Final rank: {}", rank);
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

## Related Crates

- **cyphertrade-bussines-events-contracts**: Base abstraction for business events (used by missions system)

## Versioning

This project follows semantic versioning. Breaking changes to event structures will result in a major version bump.
