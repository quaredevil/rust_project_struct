# Project Struct Base

A generic Rust project template using hexagonal architecture with Kafka, PostgreSQL, and clean architecture principles.

## Architecture

This project follows **Hexagonal Architecture (Ports & Adapters)** with clear separation of concerns:

## Features
# Base Listener Project
### Kafka Infrastructure
- Generic Kafka producers with configurable serialization
- Generic Kafka consumers with message handlers
- Batching support for high-throughput scenarios
- Automatic retry and error handling

### Database
- PostgreSQL with SQLx for type-safe queries
- Generic repository pattern
- Connection pooling and retry logic
- Migration support (add your migrations)

### Observability
- Structured JSON logging
- Startup health checks
- Graceful shutdown with resource cleanup

## Getting Started

1. **Configure environment variables**:
   ```bash
   cp .env.example .env
   # Edit .env with your settings
   ```

2. **Setup database**:
   ```bash
   # Create your migrations in migrations/ folder
   # Run migrations
   ```

3. **Build and run**:
   ```bash
   cargo build
   cargo run
   ```

## Adding Your Domain Logic

1. **Define your entities** in `src/domain/entities/`
2. **Create DTOs** in `src/application/dtos/`
3. **Define ports** in `src/application/ports/`
4. **Implement adapters** in `src/infrastructure/`
5. **Add Kafka topics** in configuration
6. **Implement repositories** using the base `Repository<T>` trait

## Configuration

Configuration is loaded from:
- Environment variables
- `.env` file
- Config files (if using config crate features)

Key configuration sections:
- `app`: Application settings (port, etc.)
- `database`: PostgreSQL connection
- `kafka_producers`: Kafka producer configs
- `kafka_consumers`: Kafka consumer configs

## Development

## Tech Stack
# Run tests
cargo test
│   └── services/       # Application services
# Run with logs
RUST_LOG=debug cargo run
│   ├── aggregates/     # Domain aggregates
# Format code
cargo fmt
│   ├── bootstrap/      # Component initialization
# Lint
cargo clippy
│       └── utils/                       # Utilitários
│           ├── mod.rs
## Docker Support

```bash
# Build
docker build -t project-struct-base .

# Run with docker-compose
docker-compose up
```

## License
├── Dockerfile                           # Build multi-stage production
[Your License Here]
├── Dockerfile.chef                      # Build otimizado com cargo-chef
├── docker-compose.yml                   # Stack completa (app, DB, Kafka, Redis)
├── docker-compose.dev.yml               # Ambiente dev (hot reload, bind mounts)
├── Makefile                             # Comandos make (build, test, docker)
├── projectmap.yaml                      # Documentação YAML estruturada
└── README.md                            # Este arquivo

### Convenções de Organização

**Camada de Domínio (`domain/`)**
- ✅ Sem dependências de infraestrutura
- ✅ Lógica de negócio pura
- ✅ Aggregates aplicam eventos e validam invariantes
- ✅ Value objects imutáveis

**Camada de Aplicação (`application/`)**
- ✅ Define **Ports** (interfaces) que infraestrutura implementa
- ✅ Orquestra casos de uso
- ✅ DTOs para comunicação entre camadas
- ✅ Queries para read-side (CQRS)

**Camada de Infraestrutura (`infrastructure/`)**
- ✅ Implementa **Adapters** (Postgres, Kafka, Binance WS)
- ✅ Bootstrap e configuração
- ✅ Event Store (atualmente in-memory)
- ✅ Messaging e streaming
- ✅ Shutdown graceful

**Camada de Apresentação (`presentation/`)**
- ✅ HTTP REST API (Axum)
- ✅ Controllers e routers
- ✅ Conversão DTOs <-> JSON
- ✅ Middlewares (logging, auth futuro)

**Shared Kernel (`shared/`)**
- ✅ Código compartilhado entre camadas
- ✅ Traits comuns (AggregateRoot)
- ✅ Tipos transversais (EventEnvelope)
- ✅ Utilitários (datetime, validação)

### Fluxo de Dependências

```
Presentation → Application → Domain
                    ↓
             Infrastructure
```

- **Domain** não depende de ninguém (núcleo puro)
- **Application** depende apenas de Domain
- **Infrastructure** implementa contratos de Application
- **Presentation** usa Application e injeta Infrastructure via DI
