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
        let row = sqlx::query!(
            r#"
            SELECT id, name
            FROM organizations
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| {
            let mut org = Organization::new();
            org.set_id(r.id);
            org.set_name(r.name);
            org
        }))
    }
    async fn find_all(&self) -> Result<Vec<Organization>, RepositoryError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name
            FROM organizations
            ORDER BY name
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|r| {
                let mut org = Organization::new();
                org.set_id(r.id.unwrap_or(0));
                org.set_name(r.name);
                org
            })
            .collect())
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
        let name = organization.name().to_string();
        let id = organization.id() as i64;

        let rows_affected = sqlx::query!(
            r#"
            UPDATE organizations
            SET name = ?
            WHERE id = ?
            "#,
            name,
            id,
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
