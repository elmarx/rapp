use crate::db::Pool;
use diesel::migration::RunMigrationsError;
use diesel::r2d2::ConnectionManager;
use diesel::{Connection, ConnectionResult, PgConnection};
use std::time::Duration;
use thiserror::Error;

#[derive(Error, Debug)]
pub enum RappDbError {
    #[error(transparent)]
    RunMigrationsError(#[from] RunMigrationsError),
    #[error(transparent)]
    R2d2Error(#[from] r2d2::Error),

    #[error("No migrations defined")]
    NoMigrationsError,
}

type MigrationRunner = fn(&PgConnection) -> Result<(), RunMigrationsError>;

pub struct DatabaseSettings {
    database_url: String,
    migration_runner: Option<MigrationRunner>,
    /// see https://docs.rs/r2d2/0.8.7/r2d2/struct.Builder.html#method.min_idle
    min_idle: Option<u32>,
    /// see https://docs.rs/r2d2/0.8.7/r2d2/struct.Builder.html#method.idle_timeout
    idle_timeout: Option<Duration>,
    /// see https://docs.rs/r2d2/0.8.7/r2d2/struct.Builder.html#method.max_size
    max_size: u32,
}

impl DatabaseSettings {
    pub fn new(database_url: String) -> Self {
        DatabaseSettings {
            database_url,
            migration_runner: None,
            // settings are tuned to use as few connections as possible,
            // since available connections are rather limited:
            // https://cloud.google.com/sql/docs/quotas
            min_idle: Some(0),
            idle_timeout: Some(Duration::from_secs(30)),
            max_size: 2,
        }
    }

    pub fn new_with_migrations(database_url: String, func: MigrationRunner) -> Self {
        let mut s = Self::new(database_url);
        s.migration_runner = Some(func);
        s
    }

    fn get_pool(&self) -> Result<Pool, r2d2::Error> {
        let manager = ConnectionManager::<PgConnection>::new(&*self.database_url);

        Pool::builder()
            .min_idle(self.min_idle)
            .max_size(self.max_size)
            .idle_timeout(self.idle_timeout)
            .test_on_check_out(true)
            .build(manager)
    }

    /// get a database-connection pool and also run migrations, so the database/pool/connection
    /// is ready to use
    pub fn establish_pooled_connection(&self) -> Result<Pool, RappDbError> {
        let pool = self.get_pool()?;

        if let Some(r) = self.migration_runner {
            let connection = pool.get()?;
            r(&connection)?;
        }

        Ok(pool)
    }

    /// establish a single connection. Useful for tests etc.
    pub fn establish_connection(&self) -> ConnectionResult<PgConnection> {
        PgConnection::establish(&self.database_url)
    }

    pub fn run_migrations(&self, connection: &PgConnection) -> Result<(), RappDbError> {
        let migrations = self
            .migration_runner
            .ok_or(RappDbError::NoMigrationsError)?;

        migrations(connection)?;

        Ok(())
    }
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        DatabaseSettings::new("postgres://".to_string())
    }
}
