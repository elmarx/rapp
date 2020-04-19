use std::error::Error;

use actix_web::{HttpResponse, web, Resource};
use prometheus::{Encoder, TextEncoder, Registry};

fn get_metrics_body(registry: Option<&Registry>) -> Result<String, Box<dyn Error>> {
    let mut buffer = vec![];
    let encoder = TextEncoder::new();
    let metric_families = prometheus::gather();
    encoder.encode(&metric_families, &mut buffer)?;

    if let Some(r) = registry {
        let metric_families = r.gather();
        encoder.encode(&metric_families, &mut buffer)?;
    }

    Ok(String::from_utf8(buffer)?)
}

pub fn metrics_endpoint_custom_registry(custom_registry: &'static Registry) -> Resource {
    web::resource("/metrics").route(web::get().to(move || {
        match get_metrics_body(Some(custom_registry)) {
            Ok(body) => HttpResponse::Ok().content_type(prometheus::TEXT_FORMAT).body(body),
            Err(e) => {
                HttpResponse::InternalServerError().body(format!("Error generating metrics: {}", e))
            }
        }
    }))
}

pub fn metrics_endpoint() -> Resource {
    web::resource("/metrics").route(web::get().to(move || {
        match get_metrics_body(None) {
            Ok(body) => HttpResponse::Ok().content_type(prometheus::TEXT_FORMAT).body(body),
            Err(e) => {
                HttpResponse::InternalServerError().body(format!("Error generating metrics: {}", e))
            }
        }
    }))
}