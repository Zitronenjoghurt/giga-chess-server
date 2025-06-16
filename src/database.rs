use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;
use r2d2::PooledConnection;

pub mod models;
pub mod schema;
pub mod stores;

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
}
