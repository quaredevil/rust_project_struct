use serde::Deserialize;
use std::time::Duration;

/// Configuration for Kafka producers
#[derive(Debug, Clone, Deserialize)]
pub struct KafkaProducerConfig {
    pub brokers: String,
    pub topic: String,
    pub client_id: Option<String>,
    pub queue_capacity: Option<usize>,
    pub max_retry_attempts: Option<u32>,
    pub retry_backoff_ms: Option<u64>,
    pub compression: Option<String>,
    pub batch_size: Option<usize>,
    pub linger_ms: Option<u64>,
    pub acks: Option<String>,
    pub idempotence: Option<bool>,
}

impl KafkaProducerConfig {
    pub fn queue_capacity(&self) -> usize {
        self.queue_capacity.unwrap_or(1000)
    }

    pub fn max_retry_attempts(&self) -> u32 {
        self.max_retry_attempts.unwrap_or(5)
    }

    pub fn retry_backoff(&self) -> Duration {
        Duration::from_millis(self.retry_backoff_ms.unwrap_or(500))
    }

    pub fn linger_duration(&self) -> Duration {
        Duration::from_millis(self.linger_ms.unwrap_or(0))
    }
}

/// Configuration for Kafka consumers
#[derive(Debug, Clone, Deserialize)]
pub struct KafkaConsumerConfig {
    pub brokers: String,
    pub topics: Vec<String>,
    pub group_id: String,
    pub client_id: Option<String>,
    pub auto_offset_reset: Option<String>,
    pub enable_auto_commit: Option<bool>,
    pub auto_commit_interval_ms: Option<u64>,
    pub session_timeout_ms: Option<u64>,
    pub max_poll_records: Option<usize>,
}

impl KafkaConsumerConfig {
    pub fn auto_offset_reset(&self) -> String {
        self.auto_offset_reset.clone().unwrap_or_else(|| "earliest".to_string())
    }

    pub fn enable_auto_commit(&self) -> bool {
        self.enable_auto_commit.unwrap_or(true)
    }

    pub fn auto_commit_interval_ms(&self) -> u64 {
        self.auto_commit_interval_ms.unwrap_or(5000)
    }

    pub fn session_timeout_ms(&self) -> u64 {
        self.session_timeout_ms.unwrap_or(10000)
    }
}

