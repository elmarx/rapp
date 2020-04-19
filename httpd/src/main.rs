use rapp::actix_web::{App, HttpServer, web};
use rapp::http::{health, metrics_endpoint};
use rapp::http::liveness;
use rapp::http::StructuredLogger;
use rapp::RappLogger;
use rapp::slog::o;

const SERVICE_NAME: &str = "sample_httpd";

#[actix_rt::main]
async fn main() -> std::io::Result<()> {
    let root_logger = RappLogger::new(SERVICE_NAME).init();

    HttpServer::new(move || {
        App::new()
            .wrap(
                StructuredLogger::new(root_logger.new(
                    o!("log_type" => "access", "protocol" => "http", "server_name" => SERVICE_NAME),
                ))
                    .exclude("/admin/liveness")
            )
            .service(
                web::scope("/admin")
                    .service(liveness)
                    .service(health)
                    .service(metrics_endpoint())
            )
    })
        .bind("0.0.0.0:8080")?
        .run()
        .await
}