use std::{sync::Arc, time::Duration};
use async_trait::async_trait;
use rdkafka::{producer::{FutureProducer, FutureRecord, Producer}, ClientConfig};
use tokio::sync::mpsc;
use tracing::{error, info, warn};
use crate::infrastructure::messaging::kafka::{KafkaProducerConfig, KafkaMessage};
use crate::infrastructure::messaging::kafka::common::MessageSerializer;
use crate::shared::errors::{InfraResult, InfrastructureError};

/// Generic Kafka producer port trait
#[async_trait]
pub trait KafkaProducerPort<T>: Send + Sync
where
    T: Send + 'static,
{
    async fn send(&self, message: KafkaMessage<T>) -> InfraResult<()>;
    async fn send_batch(&self, messages: Vec<KafkaMessage<T>>) -> InfraResult<()> {
        for msg in messages.into_iter() {
            self.send(msg).await?;
        }
        Ok(())
    }
    async fn flush(&self) -> InfraResult<()>;
    async fn health_check(&self) -> InfraResult<()>;
    async fn shutdown(&self) -> InfraResult<()> {
        self.flush().await
    }
}

/// Generic Kafka producer implementation
pub struct KafkaProducer<T, S>
where
    S: MessageSerializer<T>,
{
    producer: FutureProducer,
    topic: String,
    tx: mpsc::Sender<KafkaMessage<T>>,
    serializer: Arc<S>,
}

impl<T, S> KafkaProducer<T, S>
where
    T: Send + 'static,
    S: MessageSerializer<T> + 'static,
{
    pub async fn bootstrap(config: KafkaProducerConfig, serializer: S) -> InfraResult<Arc<Self>> {
        let max_attempts = config.max_retry_attempts();
        let retry_delay = config.retry_backoff();

        info!(
            brokers = %config.brokers,
            topic = %config.topic,
            max_attempts,
            retry_delay_ms = retry_delay.as_millis(),
            "Bootstrapping Kafka producer"
        );

        let mut client_config = ClientConfig::new();
        client_config
            .set("bootstrap.servers", &config.brokers)
            .set("message.timeout.ms", "5000");

        if let Some(client_id) = &config.client_id {
            client_config.set("client.id", client_id);
        }

        if let Some(compression) = &config.compression {
            client_config.set("compression.type", compression);
        }

        if let Some(batch_size) = config.batch_size {
            client_config.set("batch.size", batch_size.to_string());
        }

        if let Some(linger_ms) = config.linger_ms {
            client_config.set("linger.ms", linger_ms.to_string());
        }

        if let Some(acks) = &config.acks {
            client_config.set("acks", acks);
        }

        if let Some(idempotence) = config.idempotence {
            client_config.set("enable.idempotence", idempotence.to_string());
        }

        let mut last_error = None;

        for attempt in 1..=max_attempts {
            match client_config.create::<FutureProducer>() {
                Ok(producer) => {
                    if attempt > 1 {
                        info!(attempt, "Successfully connected to Kafka after retries");
                    } else {
                        info!("Kafka producer created successfully");
                    }

                    let (tx, mut rx) = mpsc::channel(config.queue_capacity());
                    let topic = config.topic.clone();
                    let serializer = Arc::new(serializer);
                    let arc = Arc::new(Self {
                        producer,
                        topic: topic.clone(),
                        tx,
                        serializer: serializer.clone(),
                    });

                    let arc_clone = arc.clone();
                    let max_retry = config.max_retry_attempts();
                    let backoff = config.retry_backoff();

                    tokio::spawn(async move {
                        while let Some(msg) = rx.recv().await {
                            if let Err(e) = send_with_retry(&arc_clone, msg, max_retry, backoff).await {
                                error!(?e, "Kafka publish failed after retries");
                            }
                        }
                    });

                    return Ok(arc);
                }
                Err(e) => {
                    last_error = Some(e.to_string());

                    if attempt < max_attempts {
                        warn!(
                            attempt,
                            max_attempts,
                            delay_ms = retry_delay.as_millis(),
                            error = %e,
                            "Failed to create Kafka producer, retrying"
                        );
                        tokio::time::sleep(retry_delay).await;
                    } else {
                        error!(
                            attempt,
                            max_attempts,
                            error = %e,
                            "Failed to create Kafka producer after all retry attempts"
                        );
                    }
                }
            }
        }

        Err(InfrastructureError::Kafka(format!(
            "Failed to bootstrap Kafka producer after {} attempts: {}",
            max_attempts,
            last_error.unwrap_or_else(|| "unknown error".to_string())
        )))
    }
}

async fn send_with_retry<T, S>(
    producer: &Arc<KafkaProducer<T, S>>,
    message: KafkaMessage<T>,
    max_retry: u32,
    backoff: Duration,
) -> InfraResult<()>
where
    S: MessageSerializer<T>,
{
    let payload = producer.serializer.serialize(&message.value)?;
    let key = message.key.as_deref().unwrap_or("");

    for attempt in 0..=max_retry {
        let mut record = FutureRecord::to(&producer.topic)
            .payload(&payload)
            .key(key);

        // Add headers if present
        for (header_key, header_value) in &message.headers {
            record = record.headers(
                rdkafka::message::OwnedHeaders::new()
                    .insert(rdkafka::message::Header {
                        key: header_key,
                        value: Some(header_value.as_bytes()),
                    })
            );
        }

        match producer.producer.send(record, Duration::from_secs(0)).await {
            Ok(_) => return Ok(()),
            Err((e, _)) => {
                if attempt == max_retry {
                    return Err(InfrastructureError::Kafka(format!("Send error: {}", e)));
                }
                tokio::time::sleep(backoff).await;
            }
        }
    }
    Ok(())
}

#[async_trait]
impl<T, S> KafkaProducerPort<T> for KafkaProducer<T, S>
where
    T: Send + Sync + 'static,
    S: MessageSerializer<T> + Send + Sync,
{
    async fn send(&self, message: KafkaMessage<T>) -> InfraResult<()> {
        self.tx
            .send(message)
            .await
            .map_err(|e| InfrastructureError::Kafka(format!("Queue send error: {}", e)))
    }

    async fn flush(&self) -> InfraResult<()> {
        let _ = self.producer.flush(Duration::from_secs(5));
        Ok(())
    }

    async fn health_check(&self) -> InfraResult<()> {
        // Producer validated at bootstrap
        Ok(())
    }
}
