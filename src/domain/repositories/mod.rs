use async_trait::async_trait;
use uuid::Uuid;
use crate::domain::aggregates::user_aggregate::UserAggregate;
use crate::shared::errors::DomainResult;

#[async_trait]
pub trait UserAggregateRepository: Send + Sync {
    async fn load(&self, id: Uuid) -> DomainResult<Option<UserAggregate>>;
    async fn save(&self, aggregate: &mut UserAggregate) -> DomainResult<()>;
}
