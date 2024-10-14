use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use tokio::time::{interval, Duration};
use actix::prelude::*;
use serde_json::json;
use std::sync::Arc;
use dotenv::dotenv;
use std::env;

mod svckit;

// Streaming Server-Side Events (SSE)
async fn sse() -> impl Responder {
    HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(device_upgrade_stream())
}

// Stream DB updates every 30 seconds
async fn device_upgrade_stream() -> impl futures::Stream<Item = String> {
    let mut interval = interval(Duration::from_secs(30));
    futures::stream::unfold((), move |_| async {
        interval.tick().await;
        let upgrade_payload = svckit::repository_db::device_upgrade_detector().await;
        Some((upgrade_payload, ()))
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    dotenv().ok();
    let db_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    // Run the periodic upgrade task every 2 minutes
    actix::spawn(async {
        let mut interval = interval(Duration::from_secs(120));
        loop {
            interval.tick().await;
            svckit::repository_db::update_firmware().await;
        }
    });

    // Launch SSE server
    HttpServer::new(|| {
        App::new()
            .route("/events", web::get().to(sse))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
