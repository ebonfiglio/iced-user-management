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
        let job_name = job.name();

        let result = sqlx::query!(
            r#"
            INSERT INTO jobs (name)
            VALUES (?)
            "#,
            job_name
        )
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut saved_job = job.clone();
        saved_job.set_id(result.last_insert_rowid());
        Ok(saved_job)
    }

    async fn update(&self, job: &Job) -> Result<(), RepositoryError> {
        Ok(())
    }

    async fn delete(&self, id: i64) -> Result<(), RepositoryError> {
        Ok(())
    }
}
