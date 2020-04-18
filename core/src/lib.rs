mod logging;

#[cfg(feature = "web")]
#[macro_use]
pub extern crate actix_web;

#[cfg(feature = "web")]
pub mod http;

#[cfg(feature = "db")]
pub use r2d2;

#[cfg(feature = "db")]
pub mod db;

pub use logging::RappLogger;
pub use slog;

pub use slog_scope::logger;
pub use slog_scope::set_global_logger;
