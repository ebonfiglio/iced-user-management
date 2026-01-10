use crate::domain::{
    repositories::{JobRepository, RepositoryError},
    Entity, Job,
};
use async_trait::async_trait;
use sqlx::SqlitePool;

use crate::domain::User;

pub struct JobSqliteRepository {
    pool: SqlitePool,
}

impl JobSqliteRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl JobRepository for JobSqliteRepository {
    async fn find_by_id(&self, id: i64) -> Result<Option<Job>, RepositoryError> {
        Ok(Some(Job::new()))
    }
    async fn find_all(&self) -> Result<Vec<Job>, RepositoryError> {
        Ok(Vec::new())
    }

    async fn create(&self, job: &Job) -> Result<Job, RepositoryError> {
        Ok(Job::new())
    }

    async fn update(&self, job: &Job) -> Result<(), RepositoryError> {
        Ok(())
    }

    async fn delete(&self, id: i64) -> Result<(), RepositoryError> {
        Ok(())
    }
}
