use std::{sync::Arc, time::Duration, marker::PhantomData};
use async_trait::async_trait;
use rdkafka::{
    consumer::{Consumer, StreamConsumer},
    ClientConfig, Message,
};
use tokio::sync::Mutex;
use tracing::{error, info, warn};
use crate::infrastructure::messaging::kafka::KafkaConsumerConfig;
use crate::infrastructure::messaging::kafka::common::MessageDeserializer;
use crate::shared::errors::{InfraResult, InfrastructureError};

/// Generic message handler trait
#[async_trait]
pub trait MessageHandler<T>: Send + Sync {
    async fn handle(&self, message: T) -> InfraResult<()>;
}

/// Generic Kafka consumer port trait
#[async_trait]
pub trait KafkaConsumerPort: Send + Sync {
    async fn start(&self) -> InfraResult<()>;
    async fn stop(&self) -> InfraResult<()>;
    async fn health_check(&self) -> InfraResult<()>;
}

/// Generic Kafka consumer implementation
pub struct KafkaConsumer<T, D, H>
where
    D: MessageDeserializer<T>,
    H: MessageHandler<T>,
{
    consumer: Arc<StreamConsumer>,
    deserializer: Arc<D>,
    handler: Arc<H>,
    running: Arc<Mutex<bool>>,
    _phantom: PhantomData<T>,
}

impl<T, D, H> KafkaConsumer<T, D, H>
where
    T: Send + 'static,
    D: MessageDeserializer<T> + 'static,
    H: MessageHandler<T> + 'static,
{
    pub async fn bootstrap(
        config: KafkaConsumerConfig,
        deserializer: D,
        handler: H,
    ) -> InfraResult<Arc<Self>> {
        info!(
            brokers = %config.brokers,
            topics = ?config.topics,
            group_id = %config.group_id,
            "Bootstrapping Kafka consumer"
        );

        let mut client_config = ClientConfig::new();
        client_config
            .set("bootstrap.servers", &config.brokers)
            .set("group.id", &config.group_id)
            .set("auto.offset.reset", config.auto_offset_reset())
            .set("enable.auto.commit", config.enable_auto_commit().to_string())
            .set("session.timeout.ms", config.session_timeout_ms().to_string());

        if let Some(client_id) = &config.client_id {
            client_config.set("client.id", client_id);
        }

        if config.enable_auto_commit() {
            client_config.set(
                "auto.commit.interval.ms",
                config.auto_commit_interval_ms().to_string(),
            );
        }

        let consumer: StreamConsumer = client_config
            .create()
            .map_err(|e| InfrastructureError::Kafka(format!("Failed to create consumer: {}", e)))?;

        let topics: Vec<&str> = config.topics.iter().map(|s| s.as_str()).collect();
        consumer
            .subscribe(&topics)
            .map_err(|e| InfrastructureError::Kafka(format!("Failed to subscribe: {}", e)))?;

        info!("Kafka consumer created and subscribed successfully");

        Ok(Arc::new(Self {
            consumer: Arc::new(consumer),
            deserializer: Arc::new(deserializer),
            handler: Arc::new(handler),
            running: Arc::new(Mutex::new(false)),
            _phantom: PhantomData,
        }))
    }
}

#[async_trait]
impl<T, D, H> KafkaConsumerPort for KafkaConsumer<T, D, H>
where
    T: Send + Sync + 'static,
    D: MessageDeserializer<T> + Send + Sync + 'static,
    H: MessageHandler<T> + Send + Sync + 'static,
{
    async fn start(&self) -> InfraResult<()> {
        let mut running = self.running.lock().await;
        if *running {
            warn!("Consumer already running");
            return Ok(());
        }
        *running = true;
        drop(running);

        info!("Starting Kafka consumer");

        let consumer = self.consumer.clone();
        let deserializer = self.deserializer.clone();
        let handler = self.handler.clone();
        let running = self.running.clone();

        tokio::spawn(async move {
            loop {
                // Check if we should stop
                {
                    let is_running = running.lock().await;
                    if !*is_running {
                        info!("Consumer stopped");
                        break;
                    }
                }

                // Poll for messages
                match consumer.recv().await {
                    Ok(message) => {
                        if let Some(payload) = message.payload() {
                            match deserializer.deserialize(payload) {
                                Ok(msg) => {
                                    if let Err(e) = handler.handle(msg).await {
                                        error!(?e, "Error handling message");
                                    }
                                }
                                Err(e) => {
                                    error!(?e, "Error deserializing message");
                                }
                            }
                        }
                    }
                    Err(e) => {
                        error!(?e, "Kafka consumer error");
                        tokio::time::sleep(Duration::from_secs(1)).await;
                    }
                }
            }
        });

        Ok(())
    }

    async fn stop(&self) -> InfraResult<()> {
        let mut running = self.running.lock().await;
        *running = false;
        info!("Stopping Kafka consumer");
        Ok(())
    }

    async fn health_check(&self) -> InfraResult<()> {
        let running = self.running.lock().await;
        if *running {
            Ok(())
        } else {
            Err(InfrastructureError::Kafka("Consumer not running".to_string()))
        }
    }
}
