use crate::domain::{
    repositories::{JobRepository, RepositoryError},
    Entity, Job,
};
use std::sync::Arc;

#[derive(Clone)]
struct JobService {
    job_repo: Arc<dyn JobRepository>,
}

impl JobService {
    pub async fn create_job(&self, mut job: Job) -> Result<Job, JobServiceError> {
        job.validate()
            .map_err(|_| JobServiceError::ValidationError)?;

        let job = self.job_repo.create(&job).await?;

        Ok(job)
    }
}

impl std::fmt::Debug for JobService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("JobService")
            .field("job_repo", &"Arc<dyn JobRepository>")
            .finish()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum JobServiceError {
    #[error("Job validation failed")]
    ValidationError,

    #[error("Database error: {0}")]
    RepositoryError(#[from] RepositoryError),
}
