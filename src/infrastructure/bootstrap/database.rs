use sqlx::PgPool;
use tracing::{info, warn, error};
use std::time::Duration;

/// Handles database connection bootstrapping with retry logic
pub struct DatabaseBootstrap;

impl DatabaseBootstrap {
    /// Attempts to connect to PostgreSQL with retry logic
    /// Returns None if DATABASE_URL is not set or connection fails after all attempts
    pub async fn connect(
        max_attempts: u32,
        retry_delay: Duration,
    ) -> Option<PgPool> {
        let database_url = match std::env::var("DATABASE_URL") {
            Ok(url) => url,
            Err(_) => {
                info!("DATABASE_URL not set; starting without DB");
                return None;
            }
        };

        info!(
            max_attempts,
            retry_delay_ms = retry_delay.as_millis(),
            "Attempting to connect to PostgreSQL"
        );

        let mut last_error = None;

        for attempt in 1..=max_attempts {
            match PgPool::connect(&database_url).await {
                Ok(pool) => {
                    // Validate connection with a simple query
                    match sqlx::query("SELECT 1").fetch_one(&pool).await {
                        Ok(_) => {
                            if attempt > 1 {
                                info!(attempt, "PostgreSQL connection validated after retries");
                            } else {
                                info!("PostgreSQL connection validated");
                            }
                            return Some(pool);
                        }
                        Err(e) => {
                            last_error = Some(format!("validation query failed: {}", e));
                            Self::log_retry_or_fail(attempt, max_attempts, &retry_delay, &e).await;
                        }
                    }
                }
                Err(e) => {
                    last_error = Some(format!("connection failed: {}", e));
                    Self::log_retry_or_fail(attempt, max_attempts, &retry_delay, &e).await;
                }
            }
        }

        if let Some(error) = last_error {
            warn!(
                error,
                "Database connection unavailable - subscriptions will not be persisted"
            );
        }

        None
    }

    async fn log_retry_or_fail(
        attempt: u32,
        max_attempts: u32,
        retry_delay: &Duration,
        error: &impl std::fmt::Display,
    ) {
        if attempt < max_attempts {
            warn!(
                attempt,
                max_attempts,
                delay_ms = retry_delay.as_millis(),
                %error,
                "Failed to connect to PostgreSQL, retrying"
            );
            tokio::time::sleep(*retry_delay).await;
        } else {
            error!(
                attempt,
                max_attempts,
                %error,
                "Failed to connect to PostgreSQL after all attempts; continuing without DB"
            );
        }
    }
}

