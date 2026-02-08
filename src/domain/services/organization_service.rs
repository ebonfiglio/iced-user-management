use crate::domain::{
    repositories::{OrganizationRepository, RepositoryError},
    Entity, Organization,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct OrganizationService {
    org_repo: Arc<dyn OrganizationRepository>,
}

impl OrganizationService {
    pub fn new(org_repo: Arc<dyn OrganizationRepository>) -> Self {
        Self { org_repo }
    }

    pub async fn get_organization_by_id(
        &self,
        id: i64,
    ) -> Result<Option<Organization>, OrganizationServiceError> {
        Ok(self.org_repo.find_by_id(id).await?)
    }
    pub async fn create_organization(
        &self,
        mut organization: Organization,
    ) -> Result<Organization, OrganizationServiceError> {
        organization
            .validate()
            .map_err(|_| OrganizationServiceError::ValidationError)?;

        let org = self.org_repo.create(&organization).await?;

        Ok(org)
    }

    pub async fn update_organization(
        &self,
        mut organization: Organization,
    ) -> Result<(), OrganizationServiceError> {
        organization
            .validate()
            .map_err(|_| OrganizationServiceError::ValidationError)?;

        self.org_repo.update(&organization).await?;

        Ok(())
    }

    pub async fn get_all_organizations(
        &self,
    ) -> Result<Vec<Organization>, OrganizationServiceError> {
        Ok(self.org_repo.find_all().await?)
    }

    pub async fn delete_organization(&self, id: i64) -> Result<(), OrganizationServiceError> {
        self.org_repo.delete(id).await?;
        Ok(())
    }
}

impl std::fmt::Debug for OrganizationService {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("OrganizationService")
            .field("org_repo", &"Arc<dyn OrganizationRepository>")
            .finish()
    }
}

#[derive(Debug, thiserror::Error)]
pub enum OrganizationServiceError {
    #[error("Organization validation failed")]
    ValidationError,

    #[error("Database error: {0}")]
    RepositoryError(#[from] RepositoryError),
}
