use tracing::info;
use tokio::signal;

/// Handles OS signals for graceful shutdown
pub struct SignalHandler;

impl SignalHandler {
    /// Waits for shutdown signal (SIGTERM or SIGINT)
    pub async fn wait_for_shutdown() {
        #[cfg(unix)]
        {
            let mut term_signal = tokio::signal::unix::signal(
                tokio::signal::unix::SignalKind::terminate()
            ).expect("Failed to register SIGTERM handler");

            tokio::select! {
                _ = signal::ctrl_c() => {
                    info!("SIGINT (Ctrl+C) received");
                }
                _ = term_signal.recv() => {
                    info!("SIGTERM received");
                }
            }
        }

        #[cfg(not(unix))]
        {
            signal::ctrl_c().await.expect("Failed to listen for Ctrl+C");
            info!("Ctrl+C received");
        }
    }
}

