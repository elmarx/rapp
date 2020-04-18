use std::time::Duration;
use diesel::PgConnection;
use diesel::r2d2::ConnectionManager;
use crate::db::Pool;
use r2d2::Error;

pub struct DatabaseSettings {
    pub database_url: String,

    /// see https://docs.rs/r2d2/0.8.7/r2d2/struct.Builder.html#method.min_idle
    pub min_idle: Option<u32>,

    /// see https://docs.rs/r2d2/0.8.7/r2d2/struct.Builder.html#method.idle_timeout
    pub idle_timeout: Option<Duration>,

    /// see https://docs.rs/r2d2/0.8.7/r2d2/struct.Builder.html#method.max_size
    pub max_size: u32,
}



impl DatabaseSettings {
    pub fn new(database_url: String) -> Self {
        DatabaseSettings {
            database_url,
            // settings are tuned to use as few connections as possible,
            // since available connections are rather limited:
            // https://cloud.google.com/sql/docs/quotas
            min_idle: Some(0),
            idle_timeout: Some(Duration::from_secs(30)),
            max_size: 2,
        }
    }

    pub fn establish_pooled_connection(&self) -> Result<Pool, Error> {
        let manager = ConnectionManager::<PgConnection>::new(&*self.database_url);

        Pool::builder()
            .min_idle(self.min_idle)
            .max_size(self.max_size)
            .idle_timeout(self.idle_timeout)
            .test_on_check_out(true)
            .build(manager)
    }
}

impl Default for DatabaseSettings {
    fn default() -> Self {
        DatabaseSettings::new("postgres://".to_string())
    }
}
