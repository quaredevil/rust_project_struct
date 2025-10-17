use async_trait::async_trait;
use crate::shared::errors::InfraResult;

/// Generic projection store trait for read models
#[async_trait]
pub trait ProjectionStore: Send + Sync {
    /// Save or update a projection
    async fn save(&self, key: &str, data: &[u8]) -> InfraResult<()>;
    
    /// Retrieve a projection
    async fn get(&self, key: &str) -> InfraResult<Option<Vec<u8>>>;
    
    /// Delete a projection
    async fn delete(&self, key: &str) -> InfraResult<()>;
}
