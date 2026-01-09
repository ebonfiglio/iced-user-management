use crate::domain::{
    repositories::{JobRepository, OrganizationRepository, RepositoryError, UserRepository},
    Entity, User,
};
use std::sync::Arc;

pub struct UserService {
    user_repo: Arc<dyn UserRepository>,
    job_repo: Arc<dyn JobRepository>,
    org_repo: Arc<dyn OrganizationRepository>,
}

impl UserService {
    pub fn new(
        user_repo: Arc<dyn UserRepository>,
        job_repo: Arc<dyn JobRepository>,
        org_repo: Arc<dyn OrganizationRepository>,
    ) -> Self {
        Self {
            user_repo,
            job_repo,
            org_repo,
        }
    }

    pub async fn create_user(&self, mut user: User) -> Result<User, UserServiceError> {
        user.validate()
            .map_err(|_| UserServiceError::ValidationError)?;

        self.job_repo
            .find_by_id(user.job_id() as i64)
            .await?
            .ok_or(UserServiceError::JobNotFound)?;

        self.org_repo
            .find_by_id(user.organization_id() as i64)
            .await?
            .ok_or(UserServiceError::OrganizationNotFound)?;

        let saved_user = self.user_repo.create(&user).await?;

        Ok(saved_user)
    }

    pub async fn update_user(&self, mut user: User) -> Result<(), UserServiceError> {
        user.validate()
            .map_err(|_| UserServiceError::ValidationError)?;

        self.job_repo
            .find_by_id(user.job_id() as i64)
            .await?
            .ok_or(UserServiceError::JobNotFound)?;

        self.org_repo
            .find_by_id(user.organization_id() as i64)
            .await?
            .ok_or(UserServiceError::OrganizationNotFound)?;

        self.user_repo.update(&user).await?;

        Ok(())
    }

    pub async fn get_all_users(&self) -> Result<Vec<User>, UserServiceError> {
        Ok(self.user_repo.find_all().await?)
    }

    pub async fn get_user_by_id(&self, id: i64) -> Result<Option<User>, UserServiceError> {
        Ok(self.user_repo.find_by_id(id).await?)
    }

    pub async fn delete_user(&self, id: i64) -> Result<(), UserServiceError> {
        self.user_repo.delete(id).await?;
        Ok(())
    }
}

#[derive(Debug, thiserror::Error)]
pub enum UserServiceError {
    #[error("User validation failed")]
    ValidationError,

    #[error("Job not found")]
    JobNotFound,

    #[error("Organization not found")]
    OrganizationNotFound,

    #[error("User not found")]
    UserNotFound,

    #[error("Database error: {0}")]
    RepositoryError(#[from] RepositoryError),
}
