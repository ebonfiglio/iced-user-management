use crate::domain::Entity;
use async_trait::async_trait;

#[async_trait]
pub trait EntityService<T: Entity>: Send + Sync {
    type Error: std::error::Error + Send + Sync;

    async fn create(&self, entity: &T) -> Result<T, Self::Error>;
    async fn update(&self, entity: &T) -> Result<(), Self::Error>;
    async fn delete(&self, id: i64) -> Result<(), Self::Error>;
    async fn find_by_id(&self, id: i64) -> Result<Option<T>, Self::Error>;
    fn find_all(&self) -> Result<Vec<T>, Self::Error>;
}
