use std::{sync::Arc, time::Duration};
use async_trait::async_trait;
use tokio::sync::Mutex;
use tokio::time::interval;
use tracing::{debug, error, info};
use crate::infrastructure::messaging::kafka::{KafkaMessage};
use crate::infrastructure::messaging::kafka::producers::KafkaProducerPort;
use crate::shared::errors::InfraResult;

/// Batching decorator for any Kafka producer
pub struct BatchingKafkaProducer<T, P>
where
    T: Send + 'static,
    P: KafkaProducerPort<T>,
{
    inner: Arc<P>,
    buffer: Arc<Mutex<Vec<KafkaMessage<T>>>>,
    max_batch_size: usize,
    flush_interval: Duration,
}

impl<T, P> BatchingKafkaProducer<T, P>
where
    T: Send + Sync + 'static,
    P: KafkaProducerPort<T> + 'static,
{
    pub fn new(
        inner: Arc<P>,
        max_batch_size: usize,
        flush_interval: Duration,
    ) -> Arc<Self> {
        let buffer = Arc::new(Mutex::new(Vec::with_capacity(max_batch_size)));
        let producer = Arc::new(Self {
            inner: inner.clone(),
            buffer: buffer.clone(),
            max_batch_size,
            flush_interval,
        });

        // Spawn background task for periodic flushing
        let producer_clone = producer.clone();
        tokio::spawn(async move {
            let mut ticker = interval(producer_clone.flush_interval);
            ticker.set_missed_tick_behavior(tokio::time::MissedTickBehavior::Skip);

            loop {
                ticker.tick().await;
                if let Err(e) = producer_clone.flush_buffer().await {
                    error!(?e, "Error flushing batched messages");
                }
            }
        });

        info!(
            max_batch_size,
            flush_interval_ms = flush_interval.as_millis(),
            "Batching producer initialized"
        );

        producer
    }

    async fn flush_buffer(&self) -> InfraResult<()> {
        let mut buffer = self.buffer.lock().await;

        if buffer.is_empty() {
            return Ok(());
        }

        let messages = std::mem::replace(&mut *buffer, Vec::with_capacity(self.max_batch_size));
        let count = messages.len();

        drop(buffer); // Release lock before sending

        debug!(count, "Flushing batched messages");

        for msg in messages {
            self.inner.send(msg).await?;
        }

        Ok(())
    }
}

#[async_trait]
impl<T, P> KafkaProducerPort<T> for BatchingKafkaProducer<T, P>
where
    T: Send + Sync + 'static,
    P: KafkaProducerPort<T> + Send + Sync + 'static,
{
    async fn send(&self, message: KafkaMessage<T>) -> InfraResult<()> {
        let mut buffer = self.buffer.lock().await;
        buffer.push(message);

        if buffer.len() >= self.max_batch_size {
            drop(buffer); // Release lock before flush
            self.flush_buffer().await?;
        }

        Ok(())
    }

    async fn send_batch(&self, messages: Vec<KafkaMessage<T>>) -> InfraResult<()> {
        for msg in messages {
            self.send(msg).await?;
        }
        Ok(())
    }

    async fn flush(&self) -> InfraResult<()> {
        self.flush_buffer().await?;
        self.inner.flush().await
    }

    async fn health_check(&self) -> InfraResult<()> {
        self.inner.health_check().await
    }

    async fn shutdown(&self) -> InfraResult<()> {
        self.flush_buffer().await?;
        self.inner.shutdown().await
    }
}
