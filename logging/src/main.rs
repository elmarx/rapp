use crate::no_slog::log_via_log_crate;
use crate::sample_module::*;
use rapp::slog::{o, slog_info};
use rapp::{set_global_logger, RappLogger};
use std::env;

fn main() {
    // initialize a root logger
    let root_logger = RappLogger::new("logging-example")
        .with_debug_log_for("logging::sample_module")
        .init();
    // set a global logger. The logger lives as long as the guard, so make sure the guard lives as long as the main-function
    let _guard = set_global_logger(root_logger.new(o!("scope" => "global")));

    let json_log_status = if env::var("RUST_LOG_JSON")
        .map(|v| v == "1")
        .unwrap_or_default()
    {
        "RUST_JSON_LOG=1 set, logging in JSON format"
    } else {
        "RUST_JSON_LOG=1 not set, logging in compact format"
    };

    // slog supports string formatting, and additional structured fields
    slog_info!(root_logger, "Hello World. {}", json_log_status; o!("type" => "example"));

    // example for a module with enforced debug-logging (set via `.with_debug_log_for()`)
    log_debug_mode(root_logger.new(o!("scope" => "module-specific logger")));
    log_via_log_crate();
    log_global();
}

mod sample_module {
    use rapp::logger;
    use rapp::slog::{o, slog_debug, slog_info, Logger};

    pub fn log_debug_mode(logger: Logger) {
        let example_value = 42;
        slog_debug!(logger, "This is debug log"; o!("example_value" => example_value));
    }

    pub fn log_global() {
        // sometimes it's not feasible to pass logging-instances around, then you may use the global logger
        // although slog discourages this usage
        slog_info!(logger(), "Without explicitly passed logger")
    }
}

mod no_slog {
    use log::info;

    pub fn log_via_log_crate() {
        // logging via well-known `log` crate. If a library does not know about `slog`, or for your own legacy code.
        info!("Using the well-known 'log' crate")
    }
}
