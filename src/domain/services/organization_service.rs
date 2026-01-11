use crate::domain::{
    repositories::{OrganizationRepository, RepositoryError},
    Entity, Organization,
};
use std::sync::Arc;

#[derive(Clone)]
pub struct OrganizationService {
    org_repo: Arc<dyn OrganizationRepository>,
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
