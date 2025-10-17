use std::collections::HashMap;
use tokio::sync::RwLock;
use uuid::Uuid;
use serde_json::Value;
use chrono::Utc;
use async_trait::async_trait;
use crate::application::ports::event_store::EventStore;
use crate::shared::errors::InfraResult;
use crate::shared::types::{EventEnvelope, EventMetadata};
use once_cell::sync::Lazy;

static STORE: Lazy<RwLock<HashMap<Uuid, Vec<EventEnvelope<Value>>>>> = Lazy::new(|| RwLock::new(HashMap::new()));

#[derive(Clone, Default)]
pub struct InMemoryEventStore;
impl InMemoryEventStore { pub fn new() -> Self { Self } }

#[async_trait]
impl EventStore for InMemoryEventStore {
    async fn append(&self, aggregate_id: Uuid, events: Vec<(String, Value)>, metadata: EventMetadata) -> InfraResult<Vec<EventEnvelope<Value>>> {
        let mut guard = STORE.write().await;
        let stream = guard.entry(aggregate_id).or_default();
        let mut created = Vec::with_capacity(events.len());
        for (ty, payload) in events.into_iter() {
            let seq = (stream.len() as u64) + 1;
            let envelope = EventEnvelope { aggregate_id, sequence: seq, event_type: ty, payload, metadata: metadata.clone(), timestamp: Utc::now() };
            stream.push(envelope.clone());
            created.push(envelope);
        }
        Ok(created)
    }
    async fn read_stream(&self, aggregate_id: Uuid) -> InfraResult<Vec<EventEnvelope<Value>>> {
        let guard = STORE.read().await;
        Ok(guard.get(&aggregate_id).cloned().unwrap_or_default())
    }
}
