use crate::domain::{
    repositories::{OrganizationRepository, RepositoryError},
    Entity, Organization,
};

use async_trait::async_trait;
use sqlx::SqlitePool;

pub struct OrganizationSqliteRepository {
    pool: SqlitePool,
}

impl OrganizationSqliteRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl OrganizationRepository for OrganizationSqliteRepository {
    async fn find_by_id(&self, id: i64) -> Result<Option<Organization>, RepositoryError> {
        Ok(Some(Organization::new()))
    }
    async fn find_all(&self) -> Result<Vec<Organization>, RepositoryError> {
        Ok(Vec::new())
    }

    async fn create(&self, organization: &Organization) -> Result<Organization, RepositoryError> {
        let organization_name = organization.name();

        let result = sqlx::query!(
            r#"
            INSERT INTO organizations (name)
            VALUES (?)
            "#,
            organization_name
        )
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut saved_organization = organization.clone();
        saved_organization.set_id(result.last_insert_rowid());
        Ok(saved_organization)
    }

    async fn update(&self, organization: &Organization) -> Result<(), RepositoryError> {
        Ok(())
    }

    async fn delete(&self, id: i64) -> Result<(), RepositoryError> {
        Ok(())
    }
}
