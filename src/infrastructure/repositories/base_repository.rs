use async_trait::async_trait;
use sqlx::PgPool;
use crate::shared::errors::InfraResult;

/// Base repository trait for CRUD operations
///
/// This provides a generic interface that can be implemented for any entity.
/// The generic parameter `T` represents the entity type.
#[async_trait]
pub trait Repository<T>: Send + Sync {
    /// Find an entity by its ID
    async fn find_by_id(&self, id: &str) -> InfraResult<Option<T>>;

    /// Find all entities
    async fn find_all(&self) -> InfraResult<Vec<T>>;

    /// Save a new entity
    async fn save(&self, entity: &T) -> InfraResult<()>;

    /// Update an existing entity
    async fn update(&self, entity: &T) -> InfraResult<()>;

    /// Delete an entity by its ID
    async fn delete(&self, id: &str) -> InfraResult<()>;
}

/// Example base repository implementation using PostgreSQL
pub struct PostgresRepository {
    pool: PgPool,
}

impl PostgresRepository {
    pub fn new(pool: PgPool) -> Self {
        Self { pool }
    }

    pub fn pool(&self) -> &PgPool {
        &self.pool
    }
}

// Implement Repository trait for your specific entities
// Example:
// #[async_trait]
// impl Repository<YourEntity> for PostgresRepository {
//     async fn find_by_id(&self, id: &str) -> InfraResult<Option<YourEntity>> {
//         // Your implementation here
//     }
//     // ... other methods
// }

