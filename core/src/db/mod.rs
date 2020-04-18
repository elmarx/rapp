use diesel::r2d2::ConnectionManager;
use diesel::PgConnection;

mod settings;
mod url_builder;

pub type Pool = r2d2::Pool<ConnectionManager<PgConnection>>;
pub type Connection = PgConnection;

pub use settings::DatabaseSettings;
pub use url_builder::DatabaseUrlBuilder;