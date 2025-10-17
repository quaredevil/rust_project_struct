use serde::Deserialize;
use std::collections::HashMap;

#[derive(Debug, Clone, Deserialize)]
pub struct AppSettings { pub port: u16 }

#[derive(Debug, Clone, Deserialize)]
pub struct TopicsConfig {
    pub prices: String,
    pub subscription_commands: String,
    pub unsubscription_commands: String,
}

impl Default for TopicsConfig {
    fn default() -> Self {
        Self {
            prices: "crypto-listener-prices".into(),
            subscription_commands: "crypto-listener-subscription-commands".into(),
            unsubscription_commands: "crypto-listener-unsubscription-commands".into(),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct KafkaSettings {
    pub brokers: String,
    #[serde(default)]
    pub topic: String, // Deprecated: kept for backward compatibility
    pub schema_registry_url: Option<String>,
    pub publish_queue_capacity: Option<usize>,
    pub max_retry_attempts: Option<u32>,
    pub retry_backoff_ms: Option<u64>,
}

impl Default for KafkaSettings {
    fn default() -> Self {
        Self {
            brokers: "kafka:9092".into(),
            topic: "crypto-listener-prices".into(), // Default updated to new pattern
            schema_registry_url: None,
            publish_queue_capacity: Some(1000),
            max_retry_attempts: Some(5),
            retry_backoff_ms: Some(500),
        }
    }
}

#[derive(Debug, Clone, Deserialize)]
pub struct KafkaProducerSettings {
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

#[derive(Debug, Clone, Deserialize)]
pub struct KafkaConsumerSettings {
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

#[derive(Debug, Clone, Deserialize)]
pub struct MarketSettings {
    pub min_price_change_percent: Option<f64>,
    pub binance_ws_url: Option<String>,
    pub batch_interval_ms: Option<u64>,
    pub batch_max_size: Option<usize>,
    pub ws_max_reconnect_attempts: Option<u32>,
    pub ws_reconnect_delay_ms: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct DatabaseSettings {
    pub max_connect_attempts: Option<u32>,
    pub connect_retry_delay_ms: Option<u64>,
}

#[derive(Debug, Clone, Deserialize)]
pub struct Settings {
    pub app: AppSettings,
    #[serde(default)]
    pub kafka: KafkaSettings,
    #[serde(default)]
    pub topics: TopicsConfig,
    #[serde(default)]
    pub kafka_producers: HashMap<String, KafkaProducerSettings>,
    #[serde(default)]
    pub kafka_consumers: HashMap<String, KafkaConsumerSettings>,
    pub market: MarketSettings,
    pub database: DatabaseSettings,
}

impl Default for Settings {
    fn default() -> Self {
        let mut kafka_producers = HashMap::new();
        kafka_producers.insert(
            "prices".to_string(),
            KafkaProducerSettings {
                brokers: "kafka:9092".into(),
                topic: "crypto-listener-prices".into(),
                client_id: Some("crypto-listener-price-producer".into()),
                queue_capacity: Some(1000),
                max_retry_attempts: Some(5),
                retry_backoff_ms: Some(500),
                compression: Some("snappy".into()),
                batch_size: Some(16384),
                linger_ms: Some(10),
                acks: Some("all".into()),
                idempotence: Some(true),
            },
        );

        Self {
            app: AppSettings { port: 8080 },
            kafka: KafkaSettings {
                brokers: "kafka:9092".into(),
                topic: "crypto-listener-prices".into(),
                schema_registry_url: None,
                publish_queue_capacity: Some(1000),
                max_retry_attempts: Some(5),
                retry_backoff_ms: Some(500)
            },
            topics: TopicsConfig::default(),
            kafka_producers,
            kafka_consumers: HashMap::new(),
            market: MarketSettings {
                min_price_change_percent: Some(0.0),
                binance_ws_url: Some("wss://stream.binance.com:9443/ws".into()),
                batch_interval_ms: Some(200),
                batch_max_size: Some(100),
                ws_max_reconnect_attempts: Some(10),
                ws_reconnect_delay_ms: Some(5000)
            },
            database: DatabaseSettings {
                max_connect_attempts: Some(5),
                connect_retry_delay_ms: Some(2000),
            }
        }
    }
}

impl Settings {
    pub fn load() -> Self {
        // Load `.env` file if present so dotenv values populate the process environment before `config` reads it.
        let _ = dotenvy::dotenv().ok();

        // Base hierarchical loading (legacy APP__ prefix style still supported for non-Kafka groups)
        let builder = config::Config::builder().add_source(config::Environment::with_prefix("APP").separator("__"));
        let mut settings = builder.build().and_then(|c| c.try_deserialize::<Settings>()).unwrap_or_else(|_| Settings::default());

        // Helper to choose first non-empty env var among candidates
        let pick = |candidates: &[&str]| -> Option<String> { for &k in candidates { if let Ok(v) = std::env::var(k) { if !v.is_empty() { return Some(v); } } } None };

        // Preferred new names: KAFKA_* (single underscore). Legacy supported: KAFKA__* and APP__KAFKA__*
        if let Some(v) = pick(&["KAFKA_BROKERS", "KAFKA__BROKERS", "APP__KAFKA__BROKERS"]) { settings.kafka.brokers = v; }
        if let Some(v) = pick(&["KAFKA_TOPIC", "KAFKA__TOPIC", "APP__KAFKA__TOPIC"]) { settings.kafka.topic = v; }
        if let Some(v) = pick(&["KAFKA_SCHEMA_REGISTRY_URL", "KAFKA__SCHEMA_REGISTRY_URL", "APP__KAFKA__SCHEMA_REGISTRY_URL"]) { settings.kafka.schema_registry_url = Some(v); }
        if let Some(v) = pick(&["KAFKA_PUBLISH_QUEUE_CAPACITY", "KAFKA__PUBLISH_QUEUE_CAPACITY", "APP__KAFKA__PUBLISH_QUEUE_CAPACITY"]) { if let Ok(n) = v.parse() { settings.kafka.publish_queue_capacity = Some(n); } }
        if let Some(v) = pick(&["KAFKA_MAX_RETRY_ATTEMPTS", "KAFKA__MAX_RETRY_ATTEMPTS", "APP__KAFKA__MAX_RETRY_ATTEMPTS"]) { if let Ok(n) = v.parse() { settings.kafka.max_retry_attempts = Some(n); } }
        if let Some(v) = pick(&["KAFKA_RETRY_BACKOFF_MS", "KAFKA__RETRY_BACKOFF_MS", "APP__KAFKA__RETRY_BACKOFF_MS"]) { if let Ok(n) = v.parse() { settings.kafka.retry_backoff_ms = Some(n); } }

        // Load multiple topic configurations
        if let Some(v) = pick(&["KAFKA_TOPIC_PRICES"]) { settings.topics.prices = v; }
        if let Some(v) = pick(&["KAFKA_TOPIC_SUBSCRIPTION_COMMANDS"]) { settings.topics.subscription_commands = v; }
        if let Some(v) = pick(&["KAFKA_TOPIC_UNSUBSCRIPTION_COMMANDS"]) { settings.topics.unsubscription_commands = v; }

        // Market/WebSocket settings
        if let Some(v) = pick(&["MARKET_WS_MAX_RECONNECT_ATTEMPTS", "APP__MARKET__WS_MAX_RECONNECT_ATTEMPTS"]) { if let Ok(n) = v.parse() { settings.market.ws_max_reconnect_attempts = Some(n); } }
        if let Some(v) = pick(&["MARKET_WS_RECONNECT_DELAY_MS", "APP__MARKET__WS_RECONNECT_DELAY_MS"]) { if let Ok(n) = v.parse() { settings.market.ws_reconnect_delay_ms = Some(n); } }

        // Database settings
        if let Some(v) = pick(&["DATABASE_MAX_CONNECT_ATTEMPTS", "APP__DATABASE__MAX_CONNECT_ATTEMPTS"]) { if let Ok(n) = v.parse() { settings.database.max_connect_attempts = Some(n); } }
        if let Some(v) = pick(&["DATABASE_CONNECT_RETRY_DELAY_MS", "APP__DATABASE__CONNECT_RETRY_DELAY_MS"]) { if let Ok(n) = v.parse() { settings.database.connect_retry_delay_ms = Some(n); } }

        settings
    }

    /// Convert legacy KafkaSettings to new KafkaProducerConfig for backward compatibility
    pub fn get_legacy_producer_config(&self) -> crate::infrastructure::messaging::kafka::KafkaProducerConfig {
        crate::infrastructure::messaging::kafka::KafkaProducerConfig {
            brokers: self.kafka.brokers.clone(),
            topic: self.topics.prices.clone(), // Use new topics config
            client_id: Some("crypto-listener-legacy".into()),
            queue_capacity: self.kafka.publish_queue_capacity,
            max_retry_attempts: self.kafka.max_retry_attempts,
            retry_backoff_ms: self.kafka.retry_backoff_ms,
            compression: Some("snappy".into()),
            batch_size: None,
            linger_ms: None,
            acks: Some("all".into()),
            idempotence: Some(true),
        }
    }

    /// Get a specific producer configuration by name
    pub fn get_producer_config(&self, name: &str) -> Option<crate::infrastructure::messaging::kafka::KafkaProducerConfig> {
        self.kafka_producers.get(name).map(|p| crate::infrastructure::messaging::kafka::KafkaProducerConfig {
            brokers: p.brokers.clone(),
            topic: p.topic.clone(),
            client_id: p.client_id.clone(),
            queue_capacity: p.queue_capacity,
            max_retry_attempts: p.max_retry_attempts,
            retry_backoff_ms: p.retry_backoff_ms,
            compression: p.compression.clone(),
            batch_size: p.batch_size,
            linger_ms: p.linger_ms,
            acks: p.acks.clone(),
            idempotence: p.idempotence,
        })
    }

    /// Get a specific consumer configuration by name
    pub fn get_consumer_config(&self, name: &str) -> Option<crate::infrastructure::messaging::kafka::KafkaConsumerConfig> {
        self.kafka_consumers.get(name).map(|c| crate::infrastructure::messaging::kafka::KafkaConsumerConfig {
            brokers: c.brokers.clone(),
            topics: c.topics.clone(),
            group_id: c.group_id.clone(),
            client_id: c.client_id.clone(),
            auto_offset_reset: c.auto_offset_reset.clone(),
            enable_auto_commit: c.enable_auto_commit,
            auto_commit_interval_ms: c.auto_commit_interval_ms,
            session_timeout_ms: c.session_timeout_ms,
            max_poll_records: c.max_poll_records,
        })
    }
}
