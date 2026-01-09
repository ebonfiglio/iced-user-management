use crate::domain::{
    repositories::{RepositoryError, UserRepository},
    Entity, User,
};
use async_trait::async_trait;
use sqlx::SqlitePool;

pub struct UserSqliteRepository {
    pool: SqlitePool,
}

impl UserSqliteRepository {
    pub fn new(pool: SqlitePool) -> Self {
        Self { pool }
    }
}

#[async_trait]
impl UserRepository for UserSqliteRepository {
    async fn find_by_id(&self, id: i64) -> Result<Option<User>, RepositoryError> {
        let row = sqlx::query!(
            r#"
            SELECT id, name, job_id, organization_id
            FROM users
            WHERE id = ?
            "#,
            id
        )
        .fetch_optional(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(row.map(|r| {
            let mut user = User::new();
            user.set_id(r.id);
            user.set_name(r.name);
            user.set_job_id(r.job_id);
            user.set_organization_id(r.organization_id);
            user
        }))
    }

    async fn find_all(&self) -> Result<Vec<User>, RepositoryError> {
        let rows = sqlx::query!(
            r#"
            SELECT id, name, job_id, organization_id
            FROM users
            ORDER BY name
            "#
        )
        .fetch_all(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        Ok(rows
            .into_iter()
            .map(|r| {
                let mut user = User::new();
                user.set_id(r.id);
                user.set_name(r.name);
                user.set_job_id(r.job_id);
                user.set_organization_id(r.organization_id);
                user
            })
            .collect())
    }

    async fn create(&self, user: &User) -> Result<User, RepositoryError> {
        let name = user.name().to_string();
        let job_id = user.job_id() as i64;
        let org_id = user.organization_id() as i64;

        let result = sqlx::query!(
            r#"
            INSERT INTO users (name, job_id, organization_id)
            VALUES (?, ?, ?)
            "#,
            name,
            job_id,
            org_id,
        )
        .execute(&self.pool)
        .await
        .map_err(|e| RepositoryError::DatabaseError(e.to_string()))?;

        let mut saved_user = user.clone();
        saved_user.set_id(result.last_insert_rowid());
        Ok(saved_user)
    }

    async fn update(&self, user: &User) -> Result<(), RepositoryError> {
        let name = user.name().to_string();
        let job_id = user.job_id() as i64;
        let org_id = user.organization_id() as i64;
        let user_id = user.id() as i64;

        let rows_affected = sqlx::query!(
            r#"
            UPDATE users
            SET name = ?, job_id = ?, organization_id = ?
            WHERE id = ?
            "#,
            name,
            job_id,
            org_id,
            user_id
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
        let rows_affected = sqlx::query!(
            r#"
            DELETE FROM users WHERE id = ?
            "#,
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
}
