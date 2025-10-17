use std::time::Duration;
use tracing::info;

/// Handles graceful shutdown of application components
pub struct GracefulShutdown {
    shutdown_timeout: Duration,
}

impl GracefulShutdown {
    /// Creates a new graceful shutdown handler with specified timeout
    pub fn new(shutdown_timeout: Duration) -> Self {
        Self { shutdown_timeout }
    }

    /// Executes graceful shutdown sequence
    pub async fn shutdown(&self) {
        info!("Starting graceful shutdown sequence");

        // Add your shutdown logic here:
        // - Flush Kafka producers
        // - Close database connections
        // - Stop background tasks
        // - etc.

        tokio::time::sleep(self.shutdown_timeout).await;

        info!("Graceful shutdown sequence completed");
    }
}

impl Default for GracefulShutdown {
    fn default() -> Self {
        Self::new(Duration::from_secs(10))
    }
}
