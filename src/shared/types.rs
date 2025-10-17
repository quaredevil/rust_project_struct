use uuid::Uuid;

pub type EventVersion = u64;

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventMetadata {
    pub correlation_id: Option<Uuid>,
    pub causation_id: Option<Uuid>,
    pub user_id: Option<Uuid>,
}

impl Default for EventMetadata {
    fn default() -> Self { Self { correlation_id: None, causation_id: None, user_id: None } }
}

pub trait Versioned {
    fn version(&self) -> EventVersion;
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct EventEnvelope<T> {
    pub aggregate_id: Uuid,
    pub sequence: EventVersion,
    pub event_type: String,
    pub payload: T,
    pub metadata: EventMetadata,
    pub timestamp: DateTime<Utc>,
}
use chrono::{DateTime, Utc};

