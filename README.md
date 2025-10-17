# Project Struct Base

A generic Rust project template using hexagonal architecture with Kafka, PostgreSQL, and clean architecture principles.

## Overview

This is a production-ready template for building scalable Rust applications with:
- **Hexagonal Architecture** (Ports & Adapters pattern)
- **CQRS** and Event Sourcing patterns
- **Clean separation** of business logic and infrastructure
- **Generic Kafka** producers/consumers
- **PostgreSQL** with type-safe queries
- **Graceful shutdown** and observability

## Architecture

This project follows **Hexagonal Architecture (Ports & Adapters)** with clear separation of concerns:

- **Domain Layer**: Pure business logic (aggregates, entities, value objects, events)
- **Application Layer**: Use cases orchestration and port definitions
- **Infrastructure Layer**: Adapters for external systems (Kafka, PostgreSQL, HTTP)
- **Shared Layer**: Common utilities, traits, and error types

### Layer Responsibilities

**Domain (`src/domain/`)**
- ✅ Zero infrastructure dependencies
- ✅ Pure business logic
- ✅ Aggregates apply events and validate invariants
- ✅ Immutable value objects

**Application (`src/application/`)**
- ✅ Defines **Ports** (interfaces) implemented by infrastructure
- ✅ Orchestrates use cases
- ✅ DTOs for cross-layer communication
- ✅ Queries for read-side (CQRS)

**Infrastructure (`src/infrastructure/`)**
- ✅ Implements **Adapters** (PostgreSQL, Kafka, HTTP)
- ✅ Bootstrap and configuration
- ✅ Event Store and message brokers
- ✅ Graceful shutdown handling

**Shared Kernel (`src/shared/`)**
- ✅ Common code across layers
- ✅ Shared traits and types
- ✅ Utilities (datetime, validation)

### Dependency Flow

```
Presentation → Application → Domain
                    ↓
             Infrastructure
```

- **Domain** has zero dependencies (pure core)
- **Application** depends only on Domain
- **Infrastructure** implements Application contracts
- **Presentation** uses Application and injects Infrastructure via DI

## Tech Stack

- **Runtime**: Tokio async (full features)
- **Web Framework**: Axum 0.8.6 (optional)
- **Database**: PostgreSQL with SQLx (type-safe queries)
- **Messaging**: Kafka with rdkafka
- **Logging**: tracing + tracing-subscriber (structured JSON logs)
- **Config**: config + dotenvy (environment-based)
- **Error Handling**: thiserror + anyhow

## Project Structure

```
src/
├── application/
│   ├── dtos/           # Data Transfer Objects
│   ├── ports/          # Port trait definitions (interfaces)
│   ├── queries/        # Query handlers (CQRS read side)
│   └── services/       # Application services (use case orchestration)
├── domain/
│   ├── aggregates/     # Domain aggregates (aggregate roots)
│   ├── entities/       # Domain entities
│   ├── events/         # Domain events
│   ├── repositories/   # Repository interfaces
│   ├── services/       # Domain services
│   └── value_objects/  # Immutable value objects
├── infrastructure/
│   ├── bootstrap/      # Component initialization with retry logic
│   ├── config/         # Configuration management
│   ├── event_store/    # Event store implementation
│   ├── messaging/
│   │   └── kafka/      # Generic Kafka producers/consumers
│   ├── repositories/   # Repository implementations
│   ├── shutdown/       # Graceful shutdown coordination
│   └── startup/        # Logging, banner, health checks
└── shared/
    ├── errors.rs       # Common error types
    ├── traits/         # Common traits
    ├── types.rs        # Type aliases
    └── utils/          # Utilities
```

## Features

### Kafka Infrastructure
- ✅ Generic Kafka producers with configurable serialization (JSON, Avro, etc.)
- ✅ Generic Kafka consumers with pluggable message handlers
- ✅ Batching support for high-throughput scenarios
- ✅ Automatic retry and error handling
- ✅ Type-safe message processing

### Database
- ✅ PostgreSQL with SQLx for compile-time verified queries
- ✅ Generic repository pattern with `Repository<T>` trait
- ✅ Connection pooling and retry logic
- ✅ Migration support (Flyway or SQLx migrations)

### Observability
- ✅ Structured JSON logging with tracing
- ✅ Startup health checks and banner
- ✅ Graceful shutdown with resource cleanup
- ✅ Signal handling (SIGTERM, SIGINT)

### Configuration
- ✅ Environment-based configuration
- ✅ `.env` file support
- ✅ Type-safe settings with validation

## Getting Started

### Prerequisites

- Rust 1.70+ (2021 edition)
- PostgreSQL 14+
- Kafka (optional, if using messaging)
- Docker & Docker Compose (for local development)

### Initial Setup

1. **Clone and configure environment**:
   ```bash
   cp .env.example .env
   # Edit .env with your configuration
   ```

2. **Update project metadata**:
   - Edit `Cargo.toml` to change the `name` field to your project name
   - Update version and authors

3. **Create your database**:
   ```bash
   createdb your_database_name
   # Update DATABASE_URL in .env
   ```

4. **Add your migrations** (optional):
   ```bash
   mkdir migrations
   # Create your SQL migration files:
   # migrations/V001__initial_schema.sql
   # migrations/V002__your_feature.sql
   ```

5. **Build and run**:
   ```bash
   cargo build
   cargo run
   ```

## Adding Your Domain Logic

This is a **template** - you need to add your business logic. Here's how:

1. **Define your entities** in `src/domain/entities/`
   - Create domain objects with identity and business rules

2. **Create DTOs** in `src/application/dtos/`
   - Define data transfer objects for API boundaries

3. **Define ports** in `src/application/ports/`
   - Create trait interfaces for external dependencies

4. **Implement adapters** in `src/infrastructure/`
   - Implement your ports with concrete technology choices

5. **Add Kafka topics** in configuration
   - Configure producers/consumers for your messaging needs

6. **Implement repositories** using the base `Repository<T>` trait
   - Add database access for your entities

See `docs/GETTING_STARTED.md` for detailed step-by-step guide.

## Configuration

Configuration is loaded from:
- Environment variables
- `.env` file
- Config files (using config crate)

Key configuration sections:
- `app`: Application settings (name, port, etc.)
- `database`: PostgreSQL connection and pooling
- `kafka_producers`: Kafka producer configurations
- `kafka_consumers`: Kafka consumer configurations

See `.env.example` for all available options.

## Development

```bash
# Run tests
cargo test

# Run with debug logs
RUST_LOG=debug cargo run

# Format code
cargo fmt

# Lint
cargo clippy

# Check without building
cargo check
```

## Docker Support

```bash
# Build production image
docker build -t project-struct-base .

# Run with docker-compose (includes PostgreSQL, Kafka, etc.)
docker-compose up

# Development mode with hot reload
docker-compose -f docker-compose.dev.yml up
```

## Documentation

- **[Getting Started Guide](docs/GETTING_STARTED.md)** - Step-by-step tutorial
- **[Architecture Guide](docs/ARCHITECTURE.md)** - Detailed architecture documentation
- **[Project Map](projectmap.yaml)** - Structured YAML documentation

## Ready-to-Use Components

This template includes production-ready implementations:

- ✅ **Generic Kafka Producer/Consumer** - Type-safe messaging with pluggable serialization
- ✅ **Repository Pattern** - Base `Repository<T>` trait with PostgreSQL implementation
- ✅ **Database Bootstrap** - Connection pooling with automatic retry logic
- ✅ **Graceful Shutdown** - Coordinated resource cleanup on termination
- ✅ **Structured Logging** - JSON logs with tracing for observability
- ✅ **Configuration Management** - Environment-based settings with validation
- ✅ **Health Checks** - Startup health summary for all components
- ✅ **Event Store** - In-memory event store (ready for persistent implementation)

## Design Patterns

- **Hexagonal Architecture** - Clean separation of business logic and infrastructure
- **Ports & Adapters** - Interface-based dependency inversion
- **Repository Pattern** - Abstract data access
- **CQRS** - Command/Query separation
- **Event Sourcing** - Event store foundation
- **Generic Programming** - Type-safe, reusable components

## Next Steps

1. Read `docs/GETTING_STARTED.md` for a detailed tutorial
2. Define your domain entities and value objects
3. Create application ports for your use cases
4. Implement infrastructure adapters
5. Add your business logic
6. Write tests
7. Deploy!

## License

[Your License Here]

---

**Note**: This is a generic template. All domain-specific code has been removed. You need to add your business logic to make it functional for your specific use case.
