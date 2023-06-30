use axum::{
    routing::{get, post},
    http::StatusCode,
    response::IntoResponse,
    Json, Router,
};
use serde::{Deserialize, Serialize};
use std::net::SocketAddr;
use std::os::linux::raw::stat;
use serde_json::json;
use crate::{cpu_frequency, cpu_usage, init_measurements, MeasuredStats, memory_usage, network_usage_in, network_usage_out, STATS};

#[tokio::main]
async fn main() {
    // initialize tracing
    init_measurements();
    tracing_subscriber::fmt::init();

    // build our application with a route
    let app = Router::new()
        .route("/", get(root))
        .route("/all-stats", get(all_stats));

    // run our app with hyper
    // `axum::Server` is a re-export of `hyper::Server`
    let addr = SocketAddr::from(([127, 0, 0, 1], 3000));
    tracing::debug!("listening on {}", addr);
    axum::Server::bind(&addr)
        .serve(app.into_make_service())
        .await
        .unwrap();
}

async fn root() -> &'static str {
    "Hello, World!\n have some yummy stats"
}

async fn all_stats() -> Json<MeasuredStats> {
    if let Some(stats) = STATS.lock().as_mut() {
        Json(stats.latest().clone())
    } else {
        Json(MeasuredStats::default())
    }
}

#[cfg(test)]
mod tests {
    use crate::server::main;

    #[test]
    fn it_works() {
        main();
    }
}