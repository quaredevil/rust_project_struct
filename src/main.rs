use tracing::info;

use project_struct_base::infrastructure::config::settings::Settings;
use project_struct_base::infrastructure::bootstrap::DatabaseBootstrap;
use project_struct_base::infrastructure::startup::LoggingSetup;

#[tokio::main]
async fn main() {
    // Initialize logging
    LoggingSetup::initialize();

    info!("Starting application");

    // Load configuration
    let settings = Settings::load();

    // Bootstrap database
    let max_db_attempts = settings.database.max_connect_attempts.unwrap_or(5);
    let db_retry_delay = std::time::Duration::from_millis(
        settings.database.connect_retry_delay_ms.unwrap_or(2000)
    );
    let _db_pool = DatabaseBootstrap::connect(max_db_attempts, db_retry_delay).await;

    info!("Application initialized successfully");

    // TODO: Add your application logic here
    // - Initialize Kafka producers/consumers
    // - Start your domain-specific services
    // - Setup HTTP server if needed

    tokio::signal::ctrl_c().await.expect("Failed to listen for ctrl+c");
    info!("Shutting down gracefully");
}
