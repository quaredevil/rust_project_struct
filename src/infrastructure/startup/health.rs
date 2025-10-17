use sqlx::PgPool;
use tracing::info;

/// Component health status
#[derive(Debug)]
pub struct ComponentHealth {
    pub database: bool,
}

/// Health check coordination for all components
pub struct HealthSummary;

impl HealthSummary {
    /// Performs health checks on all components and returns summary
    pub async fn check(db_pool: &Option<PgPool>) -> ComponentHealth {
        let database_ok = db_pool.is_some();

        info!(
            database = database_ok,
            "Component health summary"
        );

        ComponentHealth {
            database: database_ok,
        }
    }
}
