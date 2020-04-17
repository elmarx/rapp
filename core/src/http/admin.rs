// use crate::db::Pool;
use actix_web::{web::Json, HttpRequest, HttpResponse, Responder};
use serde::Serialize;
// use std::time::Duration;

/*
#[derive(Serialize)]
struct DbState {
    connection_alive: bool,
    pool_connections: u32,
    pool_idle_connections: u32,
}
*/

#[derive(Serialize)]
struct BuildInfo<'a> {
    version: &'a str,
    git_commit: &'a str,
    #[serde(skip_serializing_if = "Option::is_none")]
    rust_version: Option<&'a str>,
    #[serde(skip_serializing_if = "Option::is_none")]
    build_version: Option<&'a str>,
}

#[derive(Serialize)]
struct HealthState<'a> {
    build: BuildInfo<'a>,
    // db: DbState,
}

#[get("liveness")]
pub async fn liveness(_req: HttpRequest) -> impl Responder {
    HttpResponse::Ok()
}

#[get("health")]
pub async fn health(/*pool: web::Data<Pool>*/) -> impl Responder {
    // try to get a connection to see if the DB is available
    // this works as long as test_on_check_out(true) is set
    // let connection_alive = pool.get_timeout(Duration::from_secs(1)).is_ok();
    // let pool_state = pool.state();

    Json(HealthState {
        build: BuildInfo {
            version: env!("CARGO_PKG_VERSION"),
            git_commit: option_env!("GIT_COMMIT").unwrap_or("HEAD"),
            rust_version: option_env!("RUST_VERSION"),
            build_version: option_env!("BUILD"),
        },
        /*
        db: DbState {
            connection_alive,
            pool_connections: pool_state.connections,
            pool_idle_connections: pool_state.idle_connections,
        },
        */
    })
}
