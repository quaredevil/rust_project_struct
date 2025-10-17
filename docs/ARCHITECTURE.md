# Architecture Guide

## Overview

This project follows **Hexagonal Architecture** (also known as Ports & Adapters pattern) with clear separation between business logic and infrastructure concerns.

## Layer Responsibilities

### Domain Layer (`src/domain/`)

**Purpose**: Pure business logic with zero infrastructure dependencies.

**Contains**:
- **Aggregates**: Consistency boundaries and aggregate roots
- **Entities**: Objects with identity
- **Value Objects**: Immutable objects defined by their attributes
- **Events**: Facts that happened in the domain
- **Repository Interfaces**: Contracts for persistence (no implementation)
- **Domain Services**: Operations that don't naturally fit in entities

**Rules**:
- No dependencies on infrastructure
- No async I/O
- Use `thiserror` for domain errors
- All types should be serializable if needed for persistence

### Application Layer (`src/application/`)

**Purpose**: Orchestrate use cases and define contracts (ports) for infrastructure.

**Contains**:
- **Ports**: Trait definitions for infrastructure adapters
- **DTOs**: Data Transfer Objects for API boundaries
- **Services**: Use case orchestration
- **Queries**: CQRS read-side operations

**Rules**:
- Depends on domain layer only
- Defines ports (traits) but doesn't implement them
- Can use async traits
- Returns `InfraResult<T>` for operations that may fail

### Infrastructure Layer (`src/infrastructure/`)

**Purpose**: Implement adapters for external systems.

**Contains**:
- **Messaging**: Kafka producers/consumers
- **Repositories**: Database implementations
- **Bootstrap**: Component initialization
- **Config**: Configuration management
- **Startup/Shutdown**: Lifecycle management

**Rules**:
- Implements application ports
- Contains all I/O operations
- Framework-specific code lives here
- Should be swappable (e.g., swap Kafka for RabbitMQ)

### Shared Layer (`src/shared/`)

**Purpose**: Cross-cutting concerns used by all layers.

**Contains**:
- Error types
- Common traits
- Utility functions
- Type aliases

**Rules**:
- Keep minimal
- No business logic
- Only truly shared code

## Key Patterns

### Ports and Adapters

**Port** (in application layer):
```rust
#[async_trait]
pub trait YourPort: Send + Sync {
    async fn operation(&self) -> InfraResult<Data>;
}
```

**Adapter** (in infrastructure layer):
```rust
pub struct YourAdapter {
    // dependencies
}

#[async_trait]
impl YourPort for YourAdapter {
    async fn operation(&self) -> InfraResult<Data> {
        // implementation
    }
}
```

### Repository Pattern

Generic repository interface:

```rust
#[async_trait]
pub trait Repository<T>: Send + Sync {
    async fn find_by_id(&self, id: &str) -> InfraResult<Option<T>>;
    async fn save(&self, entity: &T) -> InfraResult<()>;
    // ...
}
```

### Event Store Pattern

Store domain events for event sourcing:

```rust
#[async_trait]
pub trait EventStore: Send + Sync {
    async fn append(&self, event: DomainEvent) -> InfraResult<()>;
    async fn get_events(&self, aggregate_id: &str) -> InfraResult<Vec<DomainEvent>>;
}
```

### CQRS Pattern

Separate read and write models:
- **Commands**: Handled by application services, mutate state
- **Queries**: Direct database queries for read models
- **Projections**: Materialized views updated from events

## Dependency Flow

```
main.rs
  └─> Infrastructure (adapters)
       └─> Application (ports, services)
            └─> Domain (pure logic)
```

Infrastructure depends on Application (implements ports).
Application depends on Domain (uses entities, events).
Domain depends on nothing.

## Bootstrap Sequence

1. **Logging** - Initialize structured logging
2. **Config** - Load settings from environment
3. **Database** - Connect to PostgreSQL with retry
4. **Kafka** - Initialize producers/consumers
5. **Services** - Wire dependencies
6. **HTTP Server** - Start API (optional)

## Shutdown Sequence

1. **Signal Handler** - Catch SIGTERM/SIGINT
2. **Stop Accepting Requests** - HTTP server
3. **Flush Buffers** - Kafka producers
4. **Close Connections** - Database, Kafka
5. **Exit** - Graceful termination

## Error Handling

### Domain Errors
Use `thiserror` for domain-specific errors:

```rust
#[derive(Debug, thiserror::Error)]
pub enum DomainError {
    #[error("Entity not found: {0}")]
    NotFound(String),
    
    #[error("Invalid operation: {0}")]
    InvalidOperation(String),
}
```

### Infrastructure Errors

Use `InfraError` from shared layer:

```rust
pub type InfraResult<T> = Result<T, InfraError>;
```

## Testing Strategy

### Unit Tests
- Domain layer: Pure logic, no mocks needed
- Application layer: Mock ports with `mockall`

### Integration Tests
- Test infrastructure adapters with real dependencies
- Use testcontainers for PostgreSQL, Kafka

### E2E Tests
- Full application tests
- Real HTTP requests, database, message queue

## Best Practices

1. **Keep domain pure** - No infrastructure in domain layer
2. **Use ports for I/O** - All external communication via traits
3. **Fail fast** - Bootstrap validates configuration early
4. **Log structured** - JSON logs for observability
5. **Handle shutdown gracefully** - Flush buffers, close connections
6. **Make it generic** - Type parameters over concrete types
7. **Test at boundaries** - Integration tests for adapters

## Example: Adding a New Feature

1. **Domain**: Define entity/value object/event
2. **Application**: Define port trait
3. **Infrastructure**: Implement adapter
4. **Bootstrap**: Wire dependencies
5. **Test**: Write integration tests

This keeps business logic isolated and infrastructure swappable.

## Configuration

Configuration is loaded from:
- Environment variables
- `.env` file
- Config files (if using config crate features)

Key configuration sections:
- `app`: Application settings (port, etc.)

### Example `.env`:

```bash
# Application Configuration
APP_NAME=project-struct-base
APP_PORT=8080
RUST_LOG=info

# Database Configuration
DATABASE_URL=postgres://user:password@localhost:5432/dbname
DATABASE_MAX_CONNECTIONS=10
DATABASE_MAX_CONNECT_ATTEMPTS=5
DATABASE_CONNECT_RETRY_DELAY_MS=2000

# Kafka Configuration
# Producer example
KAFKA_PRODUCER_BROKERS=localhost:9092
KAFKA_PRODUCER_TOPIC=your-topic-name

# Consumer example
KAFKA_CONSUMER_BROKERS=localhost:9092
KAFKA_CONSUMER_TOPIC=your-topic-name
KAFKA_CONSUMER_GROUP_ID=your-consumer-group

# Optional: Schema Registry
# SCHEMA_REGISTRY_URL=http://localhost:8081
```
