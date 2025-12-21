use sqlx::{SqlitePool, sqlite::SqliteConnectOptions};
use std::path::PathBuf;

pub struct Database {
    pub pool: SqlitePool,
}

impl Database {
    pub async fn new(database_path: &str) -> Result<Self, sqlx::Error> {
        if let Some(parent) = PathBuf::from(database_path).parent() {
            std::fs::create_dir_all(parent).map_err(|e| sqlx::Error::Io(e))?;
        }

        let options = SqliteConnectOptions::new()
            .filename(database_path)
            .create_if_missing(true);

        let pool = SqlitePool::connect_with(options).await?;

        sqlx::migrate!("./migrations").run(&pool).await?;

        Ok(Self { pool })
    }

    pub fn pool(&self) -> &SqlitePool {
        &self.pool
    }
}

pub fn get_database_path() -> PathBuf {
    #[cfg(debug_assertions)]
    {
        PathBuf::from("./data/app.db")
    }
    #[cfg(not(debug_assertions))]
    {
        dirs::data_local_dir()
            .unwrap_or_else(|| PathBuf::from("."))
            .join("iced-user-management")
            .join("app.db")
    }
}
