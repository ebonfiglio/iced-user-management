use super::{Job, Organization, User};
use async_trait::async_trait;

#[async_trait]
pub trait UserRepository: Send + Sync {
    async fn find_by_id(&self, id: i64) -> Result<Option<User>, RepositoryError>;
    async fn find_all(&self) -> Result<Vec<User>, RepositoryError>;
    async fn create(&self, user: &User) -> Result<User, RepositoryError>;
    async fn update(&self, user: &User) -> Result<(), RepositoryError>;
    async fn delete(&self, id: i64) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait JobRepository: Send + Sync {
    async fn find_by_id(&self, id: i64) -> Result<Option<Job>, RepositoryError>;
    async fn find_all(&self) -> Result<Vec<Job>, RepositoryError>;
    async fn create(&self, job: &Job) -> Result<Job, RepositoryError>;
    async fn update(&self, job: &Job) -> Result<(), RepositoryError>;
    async fn delete(&self, id: i64) -> Result<(), RepositoryError>;
}

#[async_trait]
pub trait OrganizationRepository: Send + Sync {
    async fn find_by_id(&self, id: i64) -> Result<Option<Organization>, RepositoryError>;
    async fn find_all(&self) -> Result<Vec<Organization>, RepositoryError>;
    async fn create(&self, org: &Organization) -> Result<Organization, RepositoryError>;
    async fn update(&self, org: &Organization) -> Result<(), RepositoryError>;
    async fn delete(&self, id: i64) -> Result<(), RepositoryError>;
}

#[derive(Debug, thiserror::Error)]
pub enum RepositoryError {
    #[error("Entity not found")]
    NotFound,
    #[error("Database error: {0}")]
    DatabaseError(String),
    #[error("Constraint violation: {0}")]
    ConstraintViolation(String),
}
