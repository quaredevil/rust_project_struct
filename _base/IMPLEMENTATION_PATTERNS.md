# Padrões de Implementação - Ecossistema Crypto Trading

> Documento de referência baseado na implementação do **crypto-notifications**
> 
> Este documento captura os padrões arquiteturais, organizacionais e de código 
> estabelecidos para servir como base para todos os serviços do ecossistema.

## 📋 Índice

1. [Arquitetura Base](#arquitetura-base)
2. [Estrutura de Diretórios](#estrutura-de-diretórios)
3. [Padrão de Configuração (Settings)](#padrão-de-configuração-settings)
4. [Padrão de Inicialização (Startup)](#padrão-de-inicialização-startup)
5. [Padrão Main.rs](#padrão-mainrs)
6. [Padrão de Errors](#padrão-de-errors)
7. [Padrão de Logging](#padrão-de-logging)
8. [Padrão de Messaging (Kafka)](#padrão-de-messaging-kafka)
9. [Padrão de Ports & Adapters](#padrão-de-ports--adapters)
10. [Padrão de DTOs](#padrão-de-dtos)
11. [Padrão de Domain Layer](#padrão-de-domain-layer)
12. [Padrão de Dependency Injection](#padrão-de-dependency-injection)
13. [Docker & DevOps](#docker--devops)
14. [Convenções de Código](#convenções-de-código)

---

## 🏗️ Arquitetura Base

### Princípios Fundamentais

1. **Hexagonal Architecture (Ports & Adapters)**
   - Domain layer completamente isolado de infraestrutura
   - Application layer define contratos (ports) via traits
   - Infrastructure layer implementa adapters concretos

2. **Event-Driven Architecture**
   - Comunicação assíncrona via Kafka
   - Domain events para rastreabilidade
   - Separação clara entre commands e events

3. **Dependency Inversion**
   - Dependências sempre apontam para abstrações (traits)
   - Injeção manual via Arc<dyn Trait>
   - Zero acoplamento entre camadas

### Camadas Obrigatórias

```
src/
├── domain/           # Lógica de negócio pura (zero deps externas)
├── application/      # Casos de uso e orquestração
├── infrastructure/   # Implementações concretas (I/O)
└── shared/          # Utilities cross-cutting
```

---

## 📁 Estrutura de Diretórios

### Estrutura Padrão Completa

```rust
src/
├── lib.rs                          // Re-exports públicos do crate
├── main.rs                         // Entry point + bootstrap
│
├── domain/                         // DOMAIN LAYER
│   ├── mod.rs
│   ├── errors.rs                   // DomainError
│   ├── entities/                   // Entidades com identidade
│   │   ├── mod.rs
│   │   └── {entity_name}.rs
│   ├── value_objects/              // Value objects imutáveis
│   │   ├── mod.rs
│   │   └── {vo_name}.rs
│   ├── events/                     // Domain events
│   │   ├── mod.rs
│   │   └── {event_type}_events.rs
│   ├── aggregates/                 // Aggregates DDD (opcional)
│   │   └── mod.rs
│   ├── repositories/               // Repository traits (contratos)
│   │   └── mod.rs
│   └── services/                   // Domain services
│       └── mod.rs
│
├── application/                    // APPLICATION LAYER
│   ├── mod.rs
│   ├── ports/                      // Contratos (traits)
│   │   ├── mod.rs
│   │   └── {port_name}_port.rs
│   ├── dtos/                       // Data Transfer Objects
│   │   ├── mod.rs
│   │   ├── {dto_name}_dto.rs
│   │   └── responses.rs
│   ├── services/                   // Application services (orchestrators)
│   │   ├── mod.rs
│   │   └── {service_name}.rs
│   └── queries/                    // CQRS queries (opcional)
│       └── mod.rs
│
├── infrastructure/                 // INFRASTRUCTURE LAYER
│   ├── mod.rs
│   ├── config/                     // Configuração
│   │   ├── mod.rs
│   │   └── settings.rs
│   ├── startup/                    // Bootstrap utilities
│   │   ├── mod.rs
│   │   ├── logging.rs
│   │   ├── banner.rs
│   │   └── health.rs
│   ├── messaging/                  // Kafka + Channels
│   │   ├── mod.rs
│   │   ├── kafka/
│   │   │   ├── mod.rs
│   │   │   ├── consumer.rs
│   │   │   ├── producer.rs       // (se necessário)
│   │   │   └── handler.rs
│   │   └── {channel_name}/        // Ex: telegram/, discord/
│   │       ├── mod.rs
│   │       ├── {channel}_channel.rs
│   │       └── {channel}_formatter.rs
│   ├── repositories/               // Implementações de repositórios
│   │   ├── mod.rs
│   │   └── {repo_name}_repository.rs
│   ├── retry/                      // Retry management
│   │   ├── mod.rs
│   │   └── exponential_backoff.rs
│   ├── throttling/                 // Rate limiting
│   │   ├── mod.rs
│   │   └── redis_throttler.rs
│   ├── event_store/                // Event sourcing (se aplicável)
│   │   └── mod.rs
│   └── shutdown/                   // Graceful shutdown
│       └── mod.rs
│
└── shared/                         // SHARED KERNEL
    ├── mod.rs
    ├── errors.rs                   // ApplicationError, InfrastructureError
    ├── types.rs                    // Type aliases comuns
    ├── traits/                     // Traits compartilhados
    │   └── mod.rs
    └── utils/                      // Utilities gerais
        └── mod.rs
```

### Regras de Organização

1. **Domain Layer**
   - Nunca importar de infrastructure/
   - Apenas tipos puros e lógica de negócio
   - Value objects validam na construção

2. **Application Layer**
   - Define ports (traits) para infraestrutura
   - Orquestra domain entities
   - Não contém lógica de negócio

3. **Infrastructure Layer**
   - Implementa todos os ports
   - Todo código de I/O aqui
   - Dependências externas isoladas

---

## ⚙️ Padrão de Configuração (Settings)

### Estrutura Obrigatória

```rust
// src/infrastructure/config/settings.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Settings {
    pub app: AppSettings,
    pub kafka: KafkaSettings,
    // Adicione outras sections conforme necessário
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AppSettings {
    pub name: String,
    pub version: String,
    pub host: String,
    pub port: u16,
    pub log_level: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct KafkaSettings {
    pub brokers: String,
    pub consumer_group: String,
    pub topic_input: String,          // Tópico principal de entrada
    pub topic_output: String,         // Tópico de saída (se aplicável)
    pub auto_offset_reset: String,
    pub schema_registry_url: String,  // Para Avro
}

impl Settings {
    pub fn from_env() -> Result<Self, config::ConfigError> {
        let settings = config::Config::builder()
            // 1. Definir defaults
            .set_default("app.name", env!("CARGO_PKG_NAME"))?
            .set_default("app.version", env!("CARGO_PKG_VERSION"))?
            .set_default("app.host", "0.0.0.0")?
            .set_default("app.port", 8080)?
            .set_default("app.log_level", "info")?
            .set_default("kafka.brokers", "localhost:9092")?
            .set_default("kafka.consumer_group", format!("{}-group", env!("CARGO_PKG_NAME")))?
            .set_default("kafka.auto_offset_reset", "earliest")?
            .set_default("kafka.schema_registry_url", "http://localhost:8081")?
            
            // 2. Sobrescrever com variáveis de ambiente
            .add_source(
                config::Environment::with_prefix("APP")
                    .separator("__")      // APP__KAFKA__BROKERS
                    .try_parsing(true)    // Parse tipos (bool, int, etc)
            )
            .build()?;

        settings.try_deserialize()
    }
}
```

### Convenções de Configuração

1. **Prefixo padrão**: `APP__`
2. **Separador**: `__` (double underscore)
3. **Exemplo**: `APP__KAFKA__BROKERS=localhost:9092`
4. **Sempre ter defaults razoáveis**
5. **Version vem de Cargo.toml**: `env!("CARGO_PKG_VERSION")`

### Arquivo .env.example Obrigatório

```bash
# Application
APP__APP__NAME=crypto-notifications
APP__APP__VERSION=1.0.0
APP__APP__HOST=0.0.0.0
APP__APP__PORT=8080
APP__APP__LOG_LEVEL=info

# Kafka
APP__KAFKA__BROKERS=localhost:9092
APP__KAFKA__CONSUMER_GROUP=crypto-notifications-group
APP__KAFKA__TOPIC_INPUT=crypto_notification
APP__KAFKA__AUTO_OFFSET_RESET=earliest
APP__KAFKA__SCHEMA_REGISTRY_URL=http://localhost:8081

# Database (se aplicável)
APP__DATABASE__URL=postgresql://user:pass@localhost:5432/db

# Redis (se aplicável)
APP__REDIS__URL=redis://localhost:6379
```

---

## 🚀 Padrão de Inicialização (Startup)

### Módulo startup/ Obrigatório

```
src/infrastructure/startup/
├── mod.rs          // Re-exports
├── logging.rs      // Inicialização de logging
├── banner.rs       // ASCII art banner
└── health.rs       // Health check utilities (opcional)
```

### 1. Logging Pattern

```rust
// src/infrastructure/startup/logging.rs
use tracing_subscriber::{fmt, layer::SubscriberExt, util::SubscriberInitExt, EnvFilter};

/// Initializes the structured logging system
pub fn init_logging(log_level: &str) {
    let env_filter = EnvFilter::try_from_default_env()
        .unwrap_or_else(|_| EnvFilter::new(log_level));

    tracing_subscriber::registry()
        .with(env_filter)
        .with(
            fmt::layer()
                .with_target(true)       // Mostra o módulo
                .with_thread_ids(true)   // Mostra thread ID
                .with_file(true)         // Mostra arquivo
                .with_line_number(true)  // Mostra linha
        )
        .init();
}
```

**Características obrigatórias**:
- Usar `tracing` (não `env_logger`)
- Formato estruturado
- Respeitar `RUST_LOG` env var como override
- Incluir: target, thread_ids, file, line_number

### 2. Banner Pattern

```rust
// src/infrastructure/startup/banner.rs
use crate::infrastructure::Settings;
use tracing::info;

pub fn print_banner(settings: &Settings) {
    let banner = r#"
╔═══════════════════════════════════════════════════════════╗
║                                                           ║
║   [ASCII ART DO SEU SERVIÇO AQUI]                        ║
║                                                           ║
╚═══════════════════════════════════════════════════════════╝
"#;

    println!("{}", banner);
    info!("🚀 {} v{}", settings.app.name, settings.app.version);
    info!("📡 Kafka brokers: {}", settings.kafka.brokers);
    info!("🌐 HTTP server: {}:{}", settings.app.host, settings.app.port);
    // Adicione mais informações relevantes
}
```

**Características**:
- ASCII art opcional mas recomendado
- Sempre mostrar: nome, versão, endpoints principais
- Usar emojis para categorizar informações

### 3. Module Re-exports

```rust
// src/infrastructure/startup/mod.rs
pub mod banner;
pub mod logging;

pub use banner::print_banner;
pub use logging::init_logging;
```

---

## 🎯 Padrão Main.rs

### Template Padrão

```rust
// src/main.rs
use std::sync::Arc;
use tracing::{error, info};

// Imports do seu serviço
use crypto_your_service::application::services::YourOrchestrator;
use crypto_your_service::infrastructure::config::Settings;
use crypto_your_service::infrastructure::messaging::KafkaConsumer;
// ... outros imports

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // ========================================
    // 1. CONFIGURAÇÃO
    // ========================================
    
    // Carrega .env (development)
    dotenvy::dotenv().ok();

    // Carrega settings
    let settings = Settings::from_env()?;

    // ========================================
    // 2. LOGGING
    // ========================================
    
    crypto_your_service::infrastructure::startup::init_logging(&settings.app.log_level);
    
    // ========================================
    // 3. BANNER
    // ========================================
    
    crypto_your_service::infrastructure::startup::print_banner(&settings);

    // ========================================
    // 4. INICIALIZAÇÃO DE COMPONENTES
    // ========================================
    
    info!("Initializing infrastructure components...");

    // Crie adapters (ordem: do mais interno para o mais externo)
    
    // Exemplo: External APIs
    let external_adapter = Arc::new(ExternalApiAdapter::new(&settings)?);
    
    // Exemplo: Database
    let db_pool = Arc::new(create_db_pool(&settings).await?);
    
    // Exemplo: Redis
    let redis_client = Arc::new(RedisClient::new(&settings.redis.url)?);
    
    // Exemplo: Business logic components
    let orchestrator = Arc::new(YourOrchestrator::new(
        external_adapter.clone(),
        db_pool.clone(),
        redis_client.clone(),
    ));

    // Handler
    let handler = Arc::new(MessageHandler::new(orchestrator.clone()));

    // Kafka Consumer
    let consumer = KafkaConsumer::new(&settings)?;

    info!("✅ All components initialized successfully");

    // ========================================
    // 5. INICIAR PROCESSAMENTO
    // ========================================
    
    info!("🎧 Starting to consume messages from Kafka...");

    let handler_clone = handler.clone();
    let consume_task = tokio::spawn(async move {
        if let Err(e) = consumer
            .consume_messages(|payload| {
                let handler = handler_clone.clone();
                async move { handler.handle(payload).await }
            })
            .await
        {
            error!("Kafka consumer error: {}", e);
        }
    });

    // ========================================
    // 6. GRACEFUL SHUTDOWN
    // ========================================
    
    tokio::signal::ctrl_c().await?;
    info!("Received shutdown signal, stopping...");

    consume_task.abort();

    info!("👋 Shutdown complete");
    Ok(())
}
```

### Sequência de Bootstrap Obrigatória

1. **Configuração** (dotenvy + Settings)
2. **Logging** (antes de qualquer log)
3. **Banner** (visual feedback)
4. **Componentes** (injeção de dependências)
5. **Tasks** (spawn de workers)
6. **Shutdown** (graceful com SIGINT/SIGTERM)

### Padrões Obrigatórios

- ✅ `#[tokio::main]` para async runtime
- ✅ `Result<(), Box<dyn std::error::Error>>` como retorno
- ✅ `Arc<T>` para compartilhar entre tasks
- ✅ Comentários separando seções (como no template)
- ✅ Log de cada fase do bootstrap
- ✅ Graceful shutdown com `tokio::signal::ctrl_c()`

---

## 🚨 Padrão de Errors

### Estrutura de Errors Hierárquica

```rust
// src/shared/errors.rs
use thiserror::Error;

/// Domain-level errors (business logic)
#[derive(Debug, Error)]
pub enum DomainError {
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Entity not found: {0}")]
    NotFound(String),
    
    #[error("Business rule violation: {0}")]
    BusinessRule(String),
}

/// Application-level errors (orchestration)
#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("Domain error: {0}")]
    Domain(#[from] DomainError),
    
    #[error("Validation error: {0}")]
    Validation(String),
    
    #[error("Throttled, retry after {retry_after} seconds")]
    Throttled { retry_after: u64 },
    
    #[error("Kafka error: {0}")]
    Kafka(String),
    
    #[error("Redis error: {0}")]
    Redis(String),
    
    #[error("Configuration error: {0}")]
    Configuration(String),
    
    #[error("Internal error: {0}")]
    Internal(String),
}

/// Infrastructure-level errors (I/O)
#[derive(Debug, Error)]
pub enum InfrastructureError {
    #[error("Database error: {0}")]
    Database(String),
    
    #[error("Network error: {0}")]
    Network(String),
    
    #[error("Serialization error: {0}")]
    Serialization(String),
}

// Type aliases para conveniência
pub type DomainResult<T> = Result<T, DomainError>;
pub type AppResult<T> = Result<T, ApplicationError>;
pub type InfraResult<T> = Result<T, InfrastructureError>;

// Helper constructors
impl ApplicationError {
    pub fn kafka(msg: impl Into<String>) -> Self {
        Self::Kafka(msg.into())
    }
    
    pub fn redis(msg: impl Into<String>) -> Self {
        Self::Redis(msg.into())
    }
    
    pub fn validation(msg: impl Into<String>) -> Self {
        Self::Validation(msg.into())
    }
}

// Conversões automáticas de libs externas
impl From<rdkafka::error::KafkaError> for ApplicationError {
    fn from(e: rdkafka::error::KafkaError) -> Self {
        Self::Kafka(e.to_string())
    }
}

impl From<redis::RedisError> for ApplicationError {
    fn from(e: redis::RedisError) -> Self {
        Self::Redis(e.to_string())
    }
}
```

### Regras de Error Handling

1. **Use thiserror** para definir errors
2. **Hierarquia clara**: Domain → Application → Infrastructure
3. **Type aliases** para Result<T, E>
4. **Helper constructors** para errors comuns
5. **From implementations** para conversões automáticas
6. **Mensagens descritivas** sempre

---

## 📝 Padrão de Logging

### Níveis de Log

```rust
use tracing::{trace, debug, info, warn, error};

// TRACE: Detalhes extremamente verbosos (raramente usado)
trace!("Raw payload: {:?}", raw_data);

// DEBUG: Informações de debugging úteis em dev
debug!(
    notification_id = %id,
    formatted_length = len,
    "Message formatted successfully"
);

// INFO: Fluxo normal da aplicação
info!(
    service = "kafka-consumer",
    topic = %topic,
    "Starting to consume messages"
);

// WARN: Situações recuperáveis mas anormais
warn!(
    notification_id = %id,
    retry_after = seconds,
    "Notification throttled"
);

// ERROR: Erros que impedem operação
error!(
    notification_id = %id,
    error = %e,
    "Failed to deliver notification"
);
```

### Structured Logging Pattern

```rust
// ❌ NÃO FAZER (string formatting)
info!("Processing notification {} for user {}", notif_id, user_id);

// ✅ FAZER (structured fields)
info!(
    notification_id = %notif_id,
    user_id = %user_id,
    "Processing notification"
);
```

### Log Levels para Diferentes Ambientes

- **Development**: `debug` ou `trace`
- **Staging**: `info`
- **Production**: `info` ou `warn`

---

## 📨 Padrão de Messaging (Kafka)

### Consumer Pattern

```rust
// src/infrastructure/messaging/kafka/consumer.rs
use crate::infrastructure::config::Settings;
use crate::shared::errors::ApplicationError;
use rdkafka::config::ClientConfig;
use rdkafka::consumer::{Consumer, StreamConsumer};
use rdkafka::Message;
use std::time::Duration;
use tracing::{debug, error, info, warn};

pub struct KafkaConsumer {
    consumer: StreamConsumer,
}

impl KafkaConsumer {
    pub fn new(settings: &Settings) -> Result<Self, ApplicationError> {
        let consumer: StreamConsumer = ClientConfig::new()
            .set("bootstrap.servers", &settings.kafka.brokers)
            .set("group.id", &settings.kafka.consumer_group)
            .set("enable.auto.commit", "true")
            .set("auto.offset.reset", &settings.kafka.auto_offset_reset)
            .set("session.timeout.ms", "6000")
            .set("enable.partition.eof", "false")
            .create()
            .map_err(|e| ApplicationError::kafka(format!("Failed to create consumer: {}", e)))?;

        let topics = vec![settings.kafka.topic_input.as_str()];
        consumer
            .subscribe(&topics)
            .map_err(|e| ApplicationError::kafka(format!("Failed to subscribe: {}", e)))?;

        info!("Kafka consumer initialized for topic: {}", settings.kafka.topic_input);

        Ok(Self { consumer })
    }

    pub async fn consume_messages<F, Fut>(&self, handler: F) -> Result<(), ApplicationError>
    where
        F: Fn(String) -> Fut,
        Fut: std::future::Future<Output = Result<(), ApplicationError>>,
    {
        info!("Starting to consume messages from Kafka...");

        loop {
            match self.consumer.recv().await {
                Ok(message) => {
                    let payload = match message.payload_view::<str>() {
                        Some(Ok(payload)) => payload,
                        Some(Err(e)) => {
                            error!("Error deserializing message payload: {:?}", e);
                            continue;
                        }
                        None => {
                            warn!("Empty message payload");
                            continue;
                        }
                    };

                    info!(
                        topic = message.topic(),
                        partition = message.partition(),
                        offset = message.offset(),
                        payload_size = payload.len(),
                        "Received Kafka message"
                    );

                    if let Err(e) = handler(payload.to_string()).await {
                        error!(
                            topic = message.topic(),
                            offset = message.offset(),
                            error = %e,
                            "Failed to handle message"
                        );
                    }
                }
                Err(e) => {
                    error!("Kafka consumer error: {}", e);
                    tokio::time::sleep(Duration::from_secs(1)).await;
                }
            }
        }
    }
}
```

### Handler Pattern

```rust
// src/infrastructure/messaging/kafka/handler.rs
use crate::application::dtos::YourMessageDto;
use crate::application::services::YourOrchestrator;
use crate::shared::errors::ApplicationError;
use std::sync::Arc;
use tracing::{error, info};

pub struct MessageHandler {
    orchestrator: Arc<YourOrchestrator>,
}

impl MessageHandler {
    pub fn new(orchestrator: Arc<YourOrchestrator>) -> Self {
        Self { orchestrator }
    }

    pub async fn handle(&self, payload: String) -> Result<(), ApplicationError> {
        // 1. Deserialize
        let dto: YourMessageDto = serde_json::from_str(&payload)
            .map_err(|e| ApplicationError::validation(format!("Invalid JSON: {}", e)))?;

        // 2. Validate
        let validated = dto.validate()?;

        // 3. Convert to domain
        let domain_entity = validated.to_domain()?;

        // 4. Process
        self.orchestrator.process(domain_entity).await?;

        info!("Message processed successfully");
        Ok(())
    }
}
```

---

## 🔌 Padrão de Ports & Adapters

### Definição de Port (Trait)

```rust
// src/application/ports/your_port.rs
use crate::domain::entities::YourEntity;
use crate::shared::errors::ApplicationError;
use async_trait::async_trait;

#[async_trait]
pub trait YourPort: Send + Sync {
    /// Does something with the entity
    async fn do_something(&self, entity: &YourEntity) -> Result<(), ApplicationError>;
    
    /// Health check for this port
    async fn health_check(&self) -> Result<(), ApplicationError>;
    
    /// Name identifier
    fn name(&self) -> &'static str;
}
```

### Implementação de Adapter

```rust
// src/infrastructure/adapters/your_adapter.rs
use crate::application::ports::YourPort;
use crate::domain::entities::YourEntity;
use crate::shared::errors::ApplicationError;
use async_trait::async_trait;

pub struct YourAdapter {
    // Configuration and clients
    client: reqwest::Client,
    api_url: String,
}

impl YourAdapter {
    pub fn new(api_url: String) -> Self {
        Self {
            client: reqwest::Client::new(),
            api_url,
        }
    }
}

#[async_trait]
impl YourPort for YourAdapter {
    async fn do_something(&self, entity: &YourEntity) -> Result<(), ApplicationError> {
        // Implementation
        Ok(())
    }
    
    async fn health_check(&self) -> Result<(), ApplicationError> {
        // Check connectivity
        Ok(())
    }
    
    fn name(&self) -> &'static str {
        "YourAdapter"
    }
}
```

### Convenções de Ports

1. Sempre `async_trait` para traits async
2. Bounds: `Send + Sync`
3. Retornar `Result<T, ApplicationError>`
4. Incluir `health_check()` e `name()`
5. Documentar cada método com `///`

---

## 📦 Padrão de DTOs

### DTO com Validação

```rust
// src/application/dtos/your_message_dto.rs
use serde::{Deserialize, Serialize};
use crate::shared::errors::ApplicationError;
use crate::domain::value_objects::*;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct YourMessageDto {
    pub field1: String,
    pub field2: Option<String>,
    pub timestamp: String,
}

/// Validated DTO após conversão para tipos de domínio
pub struct ValidatedYourMessage {
    pub field1: DomainType1,
    pub field2: Option<DomainType2>,
    pub timestamp: chrono::DateTime<chrono::Utc>,
}

impl YourMessageDto {
    /// Valida e converte para tipos de domínio
    pub fn validate(self) -> Result<ValidatedYourMessage, ApplicationError> {
        // Validações
        if self.field1.is_empty() {
            return Err(ApplicationError::validation("field1 cannot be empty"));
        }

        // Conversões
        let field1 = DomainType1::from_str(&self.field1)?;
        let field2 = self.field2.map(|v| DomainType2::from_str(&v)).transpose()?;
        let timestamp = chrono::DateTime::parse_from_rfc3339(&self.timestamp)
            .map_err(|e| ApplicationError::validation(format!("Invalid timestamp: {}", e)))?
            .with_timezone(&chrono::Utc);

        Ok(ValidatedYourMessage {
            field1,
            field2,
            timestamp,
        })
    }
}
```

### Response DTOs

```rust
// src/application/dtos/responses.rs
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
pub struct ApiResponse<T> {
    pub success: bool,
    pub data: Option<T>,
    pub error: Option<String>,
    pub timestamp: String,
}

impl<T> ApiResponse<T> {
    pub fn success(data: T) -> Self {
        Self {
            success: true,
            data: Some(data),
            error: None,
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }

    pub fn error(error: String) -> Self {
        Self {
            success: false,
            data: None,
            error: Some(error),
            timestamp: chrono::Utc::now().to_rfc3339(),
        }
    }
}
```

---

## 🏛️ Padrão de Domain Layer

### Value Object Pattern

```rust
// src/domain/value_objects/your_vo.rs
use serde::{Deserialize, Serialize};
use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum YourValueObject {
    Variant1,
    Variant2,
    Variant3,
}

impl YourValueObject {
    /// Lista todos os valores possíveis
    pub fn all() -> Vec<Self> {
        vec![Self::Variant1, Self::Variant2, Self::Variant3]
    }

    /// Converte para string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Variant1 => "variant1",
            Self::Variant2 => "variant2",
            Self::Variant3 => "variant3",
        }
    }

    /// Parse de string
    pub fn from_str(s: &str) -> Result<Self, DomainError> {
        match s.to_lowercase().as_str() {
            "variant1" => Ok(Self::Variant1),
            "variant2" => Ok(Self::Variant2),
            "variant3" => Ok(Self::Variant3),
            _ => Err(DomainError::Validation(format!("Invalid value: {}", s))),
        }
    }
}

impl fmt::Display for YourValueObject {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.as_str())
    }
}
```

### Entity Pattern

```rust
// src/domain/entities/your_entity.rs
use crate::domain::value_objects::*;
use chrono::{DateTime, Utc};
use uuid::Uuid;

#[derive(Debug, Clone)]
pub struct YourEntity {
    pub id: Uuid,
    pub field1: YourValueObject,
    pub field2: String,
    pub status: EntityStatus,
    pub created_at: DateTime<Utc>,
    pub updated_at: DateTime<Utc>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum EntityStatus {
    Pending,
    Processing,
    Completed,
    Failed,
}

impl YourEntity {
    /// Cria uma nova entidade
    pub fn new(field1: YourValueObject, field2: String) -> Self {
        let now = Utc::now();
        Self {
            id: Uuid::new_v4(),
            field1,
            field2,
            status: EntityStatus::Pending,
            created_at: now,
            updated_at: now,
        }
    }

    /// Transição de estado
    pub fn mark_as_completed(&mut self) {
        self.status = EntityStatus::Completed;
        self.updated_at = Utc::now();
    }

    /// Predicados de negócio
    pub fn can_process(&self) -> bool {
        self.status == EntityStatus::Pending
    }
}
```

---

## 💉 Padrão de Dependency Injection

### Manual DI via Arc<dyn Trait>

```rust
// No main.rs
use std::sync::Arc;

// 1. Criar adapters concretos
let adapter1 = Arc::new(ConcreteAdapter1::new(config));
let adapter2 = Arc::new(ConcreteAdapter2::new(config));

// 2. Injetar em orchestrators
let orchestrator = Arc::new(YourOrchestrator::new(
    adapter1.clone(),  // Arc pode ser clonado barato
    adapter2.clone(),
));

// 3. Passar para handlers
let handler = Arc::new(Handler::new(orchestrator.clone()));

// 4. Usar em tasks
let handler_clone = handler.clone();
tokio::spawn(async move {
    handler_clone.process().await;
});
```

### Orchestrator com DI

```rust
// src/application/services/your_orchestrator.rs
use std::sync::Arc;
use crate::application::ports::*;

pub struct YourOrchestrator {
    port1: Arc<dyn Port1>,
    port2: Arc<dyn Port2>,
}

impl YourOrchestrator {
    pub fn new(
        port1: Arc<dyn Port1>,
        port2: Arc<dyn Port2>,
    ) -> Self {
        Self { port1, port2 }
    }

    pub async fn process(&self, entity: YourEntity) -> Result<(), ApplicationError> {
        // Usa os ports injetados
        self.port1.do_something(&entity).await?;
        self.port2.do_something_else(&entity).await?;
        Ok(())
    }
}
```

---

## 🐳 Docker & DevOps

### Dockerfile Multi-Stage

```dockerfile
# Build stage
FROM rust:1.75 as builder
WORKDIR /app
COPY Cargo.toml Cargo.lock ./
COPY src ./src
RUN cargo build --release

# Runtime stage
FROM debian:bookworm-slim
RUN apt-get update && apt-get install -y \
    ca-certificates \
    && rm -rf /var/lib/apt/lists/*

WORKDIR /app
COPY --from=builder /app/target/release/crypto-your-service /app/

# Non-root user
RUN useradd -m -u 1001 appuser
USER appuser

EXPOSE 8080
CMD ["/app/crypto-your-service"]
```

### docker-compose.yml Pattern

```yaml
version: '3.8'

services:
  your-service:
    build: .
    container_name: crypto-your-service
    environment:
      APP__APP__LOG_LEVEL: info
      APP__KAFKA__BROKERS: kafka:9092
      APP__REDIS__URL: redis://redis:6379
    depends_on:
      - kafka
      - redis
    networks:
      - crypto-network

  kafka:
    image: confluentinc/cp-kafka:7.5.0
    environment:
      KAFKA_BROKER_ID: 1
      KAFKA_ZOOKEEPER_CONNECT: zookeeper:2181
      KAFKA_ADVERTISED_LISTENERS: PLAINTEXT://kafka:9092
      KAFKA_OFFSETS_TOPIC_REPLICATION_FACTOR: 1
    depends_on:
      - zookeeper
    networks:
      - crypto-network

  zookeeper:
    image: confluentinc/cp-zookeeper:7.5.0
    environment:
      ZOOKEEPER_CLIENT_PORT: 2181
    networks:
      - crypto-network

  redis:
    image: redis:7-alpine
    networks:
      - crypto-network

networks:
  crypto-network:
    driver: bridge
```

### Makefile Pattern

```makefile
.PHONY: build run test docker-up docker-down

build:
	cargo build

build-release:
	cargo build --release

run:
	cargo run

test:
	cargo test

check:
	cargo check

clippy:
	cargo clippy -- -D warnings

fmt:
	cargo fmt

docker-up:
	docker-compose up -d

docker-down:
	docker-compose down

docker-logs:
	docker-compose logs -f your-service

kafka-create-topic:
	docker exec -it crypto-kafka kafka-topics \
		--create --topic your_topic \
		--bootstrap-server localhost:9092 \
		--partitions 3 \
		--replication-factor 1
```

---

## 📐 Convenções de Código

### Naming Conventions

1. **Modules, files, functions**: `snake_case`
2. **Types, Traits, Enums**: `PascalCase`
3. **Constants**: `SCREAMING_SNAKE_CASE`
4. **Lifetimes**: `'a`, `'b`, etc (curtos)

### Code Organization

```rust
// Ordem de imports
use std::sync::Arc;              // 1. Std library
use tokio::sync::Mutex;          // 2. External crates
use crate::domain::entities::*;  // 3. Crate interno
use super::utils::*;             // 4. Módulo pai

// Ordem em structs
pub struct Example {
    // 1. Public fields
    pub id: Uuid,
    
    // 2. Private fields
    status: Status,
}

// Ordem em impl
impl Example {
    // 1. Constructor(s)
    pub fn new() -> Self { }
    
    // 2. Public methods
    pub fn do_something(&self) { }
    
    // 3. Private methods
    fn internal_helper(&self) { }
}
```

### Documentation

```rust
/// Brief description of the struct/function
///
/// More detailed explanation if needed.
///
/// # Examples
///
/// ```
/// let example = Example::new();
/// ```
///
/// # Errors
///
/// Returns error if...
pub fn example() -> Result<(), Error> {
    Ok(())
}
```

---

## 🎯 Checklist de Novo Serviço

Ao criar um novo serviço do ecossistema, siga este checklist:

### Estrutura Base
- [ ] Criar estrutura de diretórios padrão (domain/application/infrastructure/shared)
- [ ] Configurar Cargo.toml com dependências comuns
- [ ] Criar lib.rs com re-exports públicos
- [ ] Criar main.rs seguindo o template

### Configuração
- [ ] Implementar Settings em infrastructure/config/settings.rs
- [ ] Criar .env.example com todas as variáveis
- [ ] Usar prefixo `APP__` para env vars

### Startup
- [ ] Implementar init_logging() em infrastructure/startup/logging.rs
- [ ] Criar banner ASCII em infrastructure/startup/banner.rs
- [ ] Exportar funções em infrastructure/startup/mod.rs

### Domain Layer
- [ ] Criar value objects necessários
- [ ] Criar entities principais
- [ ] Definir domain events
- [ ] Implementar DomainError

### Application Layer
- [ ] Definir ports (traits) necessários
- [ ] Criar DTOs de entrada/saída
- [ ] Implementar orchestrators/services
- [ ] Definir ApplicationError

### Infrastructure Layer
- [ ] Implementar KafkaConsumer
- [ ] Implementar MessageHandler
- [ ] Implementar adapters para ports
- [ ] Configurar retry/throttling se necessário

### DevOps
- [ ] Criar Dockerfile multi-stage
- [ ] Criar docker-compose.yml
- [ ] Criar Makefile com comandos úteis
- [ ] Documentar no README.md

### Documentação
- [ ] README.md completo
- [ ] Schemas Avro (se aplicável)
- [ ] Diagramas de arquitetura
- [ ] Documentação de integração com ecossistema

---

## 📚 Dependencies Comuns

### Cargo.toml Base

```toml
[package]
name = "crypto-your-service"
version = "0.1.0"
edition = "2021"

[dependencies]
# Async runtime
tokio = { version = "1", features = ["full"] }

# Web framework (se necessário)
axum = "0.8"

# Kafka
rdkafka = "0.36"

# Redis (se necessário)
redis = { version = "0.27", features = ["tokio-comp", "connection-manager"] }

# HTTP client
reqwest = { version = "0.12", features = ["json"] }

# Serialization
serde = { version = "1", features = ["derive"] }
serde_json = "1"

# Logging
tracing = "0.1"
tracing-subscriber = { version = "0.3", features = ["env-filter"] }

# Error handling
thiserror = "1"
anyhow = "1"

# Date/Time
chrono = { version = "0.4", features = ["serde"] }

# UUID
uuid = { version = "1", features = ["v4", "serde"] }

# Configuration
config = "0.14"
dotenvy = "0.15"

# Async trait
async-trait = "0.1"

# Database (se necessário)
sqlx = { version = "0.8", features = ["runtime-tokio", "postgres", "uuid", "chrono"] }
```

---

## 🔗 Integração com Ecossistema

### Tópicos Kafka Padrão

**Nomenclatura**: `{domain}_{action}` ou `{entity_name}`

Exemplos:
- `crypto_notification` (entrada)
- `crypto_notification_delivered` (saída)
- `crypto_trade_executed` (evento)
- `crypto_signal_generated` (evento)

### Schema Registry

Todos os tópicos devem ter schemas Avro registrados em `/schemas/`:

```
schemas/
├── crypto_notification.avsc
├── crypto_notification_delivered.avsc
└── README.md
```

### Health Checks

Cada serviço deve expor:
- Endpoint `/health` (básico)
- Endpoint `/health/ready` (com dependências)

---

## 📖 Referências

- **Projeto Base**: crypto-notifications
- **Arquitetura**: Hexagonal (Ports & Adapters)
- **Padrão de Comunicação**: Event-Driven via Kafka
- **Runtime**: Tokio async
- **Logging**: tracing + tracing-subscriber

---

**Última atualização**: 2025-10-17  
**Versão do documento**: 1.0.0  
**Baseado em**: crypto-notifications v1.0.0

