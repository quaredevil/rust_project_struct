use thiserror::Error;

#[derive(Debug, Error)]
pub enum DomainError {
    #[error("validation error: {0}")] Validation(String),
    #[error("not found: {0}")] NotFound(String),
    #[error("concurrency error: {0}")] Concurrency(String),
}

#[derive(Debug, Error)]
pub enum ApplicationError {
    #[error("domain error: {0}")] Domain(#[from] DomainError),
    #[error("infrastructure error: {0}")] Infrastructure(#[from] InfrastructureError),
    #[error("serialization error: {0}")] Serde(#[from] serde_json::Error),
}

#[derive(Debug, Error)]
pub enum InfrastructureError {
    #[error("database error: {0}")] Database(String),
    #[error("event store error: {0}")] EventStore(String),
    #[error("io error: {0}")] Io(String),
    #[error("messaging error: {0}")] Messaging(String),
    #[error("websocket error: {0}")] WebSocket(String),
    #[error("kafka error: {0}")] Kafka(String),
    #[error("serialization error: {0}")] Serialization(String),
}

pub type DomainResult<T> = Result<T, DomainError>;
pub type AppResult<T> = Result<T, ApplicationError>;
pub type InfraResult<T> = Result<T, InfrastructureError>;
