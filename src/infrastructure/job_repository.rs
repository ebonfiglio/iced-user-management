use crate::domain::{
    repositories::{JobRepository, RepositoryError},
    Entity, Job,
};
use async_trait::async_trait;
use sqlx::SqlitePool;

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
        let row = sqlx::query!(
            r#"
            SELECT id, name
            FROM jobs
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| {
            let mut job = Job::new();
            job.set_id(r.id);
            job.set_name(r.name);
            job
        }))
    }
    async fn find_all(&self) -> Result<Vec<Job>, RepositoryError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name
            FROM jobs
            ORDER BY name
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|r| {
                let mut job = Job::new();
                job.set_id(r.id.unwrap_or(0));
                job.set_name(r.name);
                job
            })
            .collect())
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
        let name = job.name().to_string();
        let id = job.id() as i64;

        let rows_affected = sqlx::query!(
            r#"
            UPDATE jobs
            SET name = ?
            WHERE id = ?
            "#,
            name,
            id
        )
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?
        .rows_affected();

        if rows_affected == 0 {
            return Err(RepositoryError::NotFound);
        }

        Ok(())
    }

    async fn delete(&self, id: i64) -> Result<(), RepositoryError> {
        Ok(())
    }
}
