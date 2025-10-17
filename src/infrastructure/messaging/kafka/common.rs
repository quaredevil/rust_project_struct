use serde::{Deserialize, Serialize};
use crate::shared::errors::{InfraResult, InfrastructureError};

/// Supported serialization formats for Kafka messages
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum SerializationFormat {
    Json,
    Avro,
    Protobuf,
}

/// Generic Kafka message wrapper
#[derive(Debug, Clone)]
pub struct KafkaMessage<T> {
    pub key: Option<String>,
    pub value: T,
    pub headers: Vec<(String, String)>,
}

impl<T> KafkaMessage<T> {
    pub fn new(value: T) -> Self {
        Self {
            key: None,
            value,
            headers: Vec::new(),
        }
    }

    pub fn with_key(mut self, key: String) -> Self {
        self.key = Some(key);
        self
    }

    pub fn with_header(mut self, key: String, value: String) -> Self {
        self.headers.push((key, value));
        self
    }
}

/// Trait for serializing messages to bytes
pub trait MessageSerializer<T>: Send + Sync {
    fn serialize(&self, message: &T) -> InfraResult<Vec<u8>>;
}

/// Trait for deserializing messages from bytes
pub trait MessageDeserializer<T>: Send + Sync {
    fn deserialize(&self, bytes: &[u8]) -> InfraResult<T>;
}

/// JSON serializer implementation
pub struct JsonSerializer;

impl<T: Serialize> MessageSerializer<T> for JsonSerializer {
    fn serialize(&self, message: &T) -> InfraResult<Vec<u8>> {
        serde_json::to_vec(message)
            .map_err(|e| InfrastructureError::Serialization(format!("JSON serialize error: {}", e)))
    }
}

/// JSON deserializer implementation
pub struct JsonDeserializer;

impl<T: for<'de> Deserialize<'de>> MessageDeserializer<T> for JsonDeserializer {
    fn deserialize(&self, bytes: &[u8]) -> InfraResult<T> {
        serde_json::from_slice(bytes)
            .map_err(|e| InfrastructureError::Serialization(format!("JSON deserialize error: {}", e)))
    }
}

