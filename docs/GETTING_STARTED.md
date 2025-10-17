# Getting Started

This document provides a quick guide to start building your application using this base template.

## Prerequisites

- Rust 1.70+ (2021 edition)
- PostgreSQL 14+
- Kafka (optional, if using messaging)
- Docker & Docker Compose (for local development)

## Initial Setup

1. **Clone and configure environment**:
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

2. **Update Cargo.toml**:
   - Change the `name` field to your project name
   - Update version and authors

3. **Create your database**:
   ```bash
   createdb your_database_name
   ```

4. **Add your migrations**:
   Create SQL migration files in `migrations/` folder:
   ```
   migrations/
   ├── V001__initial_schema.sql
   ├── V002__add_your_tables.sql
   ```

## Project Structure Overview

```
src/
├── application/     # Business logic coordination
│   ├── dtos/       # Data Transfer Objects (add yours here)
│   ├── ports/      # Interface definitions (add yours here)
│   ├── queries/    # CQRS read side (add yours here)
│   └── services/   # Use case orchestration (add yours here)
├── domain/         # Pure business logic
│   ├── aggregates/ # Aggregate roots (add yours here)
│   ├── entities/   # Domain entities (add yours here)
│   ├── events/     # Domain events (add yours here)
│   └── value_objects/ # Immutable values (add yours here)
├── infrastructure/ # External adapters
│   ├── messaging/  # Kafka (already implemented)
│   ├── repositories/ # Database (base provided)
│   └── ...
└── shared/         # Common utilities
```

## Adding Your Domain

### 1. Define Your Entities

Create your domain entities in `src/domain/entities/`:

```rust
// src/domain/entities/your_entity.rs
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YourEntity {
    pub id: Uuid,
    pub name: String,
    // ... your fields
}
```

### 2. Create DTOs

Add DTOs in `src/application/dtos/`:

```rust
// src/application/dtos/your_dto.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct YourDto {
    pub field1: String,
    pub field2: i32,
}
```

### 3. Define Ports (Interfaces)

Create port traits in `src/application/ports/`:

```rust
// src/application/ports/your_port.rs
use async_trait::async_trait;
use crate::shared::errors::InfraResult;

#[async_trait]
pub trait YourPort: Send + Sync {
    async fn do_something(&self, data: String) -> InfraResult<()>;
}
```

### 4. Implement Infrastructure Adapters

Implement your ports in `src/infrastructure/`:

```rust
// src/infrastructure/your_adapter.rs
use async_trait::async_trait;
use crate::application::ports::YourPort;
use crate::shared::errors::InfraResult;

pub struct YourAdapter {
    // dependencies
}

#[async_trait]
impl YourPort for YourAdapter {
    async fn do_something(&self, data: String) -> InfraResult<()> {
        // implementation
        Ok(())
    }
}
```

### 5. Implement Repository

Use the base repository pattern:

```rust
// src/infrastructure/repositories/your_repository.rs
use async_trait::async_trait;
use sqlx::PgPool;
use crate::domain::entities::YourEntity;
use crate::infrastructure::repositories::{Repository, PostgresRepository};
use crate::shared::errors::InfraResult;

pub struct YourEntityRepository {
    base: PostgresRepository,
}

impl YourEntityRepository {
    pub fn new(pool: PgPool) -> Self {
        Self {
            base: PostgresRepository::new(pool),
        }
    }
}

#[async_trait]
impl Repository<YourEntity> for YourEntityRepository {
    async fn find_by_id(&self, id: &str) -> InfraResult<Option<YourEntity>> {
        // Your SQL query here
        todo!()
    }
    
    // ... implement other methods
}
```

## Using Kafka

### Producer Example

```rust
use base_listener::infrastructure::messaging::kafka::{
    KafkaProducer, KafkaProducerConfig, KafkaMessage
};
use base_listener::infrastructure::messaging::kafka::common::JsonSerializer;

// Your message type
#[derive(Serialize, Deserialize)]
struct YourMessage {
    field: String,
}

// Bootstrap producer
let config = KafkaProducerConfig {
    brokers: "localhost:9092".to_string(),
    topic: "your-topic".to_string(),
    // ... other config
};

let producer = KafkaProducer::bootstrap(config, JsonSerializer).await?;

// Send message
let message = KafkaMessage::new(YourMessage { field: "value".into() });
producer.send(message).await?;
```

### Consumer Example

```rust
use project_struct_base::infrastructure::messaging::kafka::{
    KafkaConsumer, KafkaConsumerConfig, MessageHandler
};

// Implement message handler
struct YourMessageHandler;

#[async_trait]
impl MessageHandler<YourMessage> for YourMessageHandler {
    async fn handle(&self, message: YourMessage) -> Result<(), Box<dyn std::error::Error>> {
        // Process your message
        Ok(())
    }
}

// Bootstrap consumer
let config = KafkaConsumerConfig {
    brokers: "localhost:9092".to_string(),
    group_id: "your-group".to_string(),
    topics: vec!["your-topic".to_string()],
    // ... other config
};

let handler = YourMessageHandler;
let consumer = KafkaConsumer::new(config, handler, JsonDeserializer);
consumer.start().await?;
```

## Running the Application

```bash
# Development
cargo run

# With logs
RUST_LOG=debug cargo run

# Production build
cargo build --release
./target/release/project-struct-base
```

## Testing

```bash
# Run all tests
cargo test

# Run specific test
cargo test test_name

# Run with output
cargo test -- --nocapture
```

## Next Steps

1. Define your domain entities and value objects
2. Create ports for your use cases
3. Implement adapters for external systems
4. Add your business logic
5. Write tests
6. Deploy!

## Need Help?

- Check the existing code in `src/infrastructure/messaging/kafka/` for Kafka examples
- See `src/infrastructure/repositories/base_repository.rs` for repository pattern
- Review `src/infrastructure/bootstrap/` for initialization patterns
