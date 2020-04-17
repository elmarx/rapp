mod logging;

#[cfg(feature = "web")]
#[macro_use]
pub extern crate actix_web;

#[cfg(feature = "web")]
pub mod http;

pub use logging::RappLogger;
pub use slog;

pub use slog_scope::logger;
pub use slog_scope::set_global_logger;
