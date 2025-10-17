use async_trait::async_trait;
use uuid::Uuid;
use serde_json::Value;
use crate::shared::errors::InfraResult;
use crate::shared::types::{EventEnvelope, EventMetadata};

#[async_trait]
pub trait EventStore: Send + Sync {
    async fn append(&self, aggregate_id: Uuid, events: Vec<(String, Value)>, metadata: EventMetadata) -> InfraResult<Vec<EventEnvelope<Value>>>;
    async fn read_stream(&self, aggregate_id: Uuid) -> InfraResult<Vec<EventEnvelope<Value>>>;
}

