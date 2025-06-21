use crate::app::error::{AppError, AppResult};
use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use diesel_migrations::{embed_migrations, EmbeddedMigrations, MigrationHarness};
use r2d2::PooledConnection;

pub mod models;
pub mod schema;
pub mod stores;
pub mod types;

pub const MIGRATIONS: EmbeddedMigrations = embed_migrations!("migrations");

#[derive(Clone)]
pub struct Database {
    connection_pool: r2d2::Pool<ConnectionManager<PgConnection>>,
}

impl Database {
    pub fn connect(url: &str) -> Result<Self, r2d2::Error> {
        Ok(Self {
            connection_pool: r2d2::Pool::new(ConnectionManager::<PgConnection>::new(url))?,
        })
    }

    pub fn get_connection(
        &self,
    ) -> Result<PooledConnection<ConnectionManager<PgConnection>>, r2d2::Error> {
        self.connection_pool.get()
    }

    pub fn clear(&self) -> AppResult<()> {
        let mut connection = self.get_connection()?;

        connection
            .revert_all_migrations(MIGRATIONS)
            .map_err(|err| AppError::DatabaseMigrationError(err.to_string()))?;

        connection
            .run_pending_migrations(MIGRATIONS)
            .map_err(|err| AppError::DatabaseMigrationError(err.to_string()))?;

        Ok(())
    }
}
