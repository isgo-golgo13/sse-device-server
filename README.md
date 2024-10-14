# SSE (Server-Sent Events) Device Upgrade Server
Rust Server-Sent Events (SSE) Device Upgrade Push Event Server to Client using Redis Cache and PostgresSQL


The SSE Device Upgrade Server is a HTTP Server that routinely sends a stream of device upgrade events to the
HTTP client. The server launches two distinct tasks to actively upgrade a device firmware version to the device DB (PostgreSQL)
and detect changes upgrade changes to the device DB to dispatch this upgrade event as a SSE payload to the device client process.


![see-device-upgrade-server-workflow]()


The following is the pre-check prototype of this production server to understand how the SSE server functions.

```rust
use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use futures::StreamExt;
use std::time::Duration;
use tokio::time::interval;
use actix_web::rt::pin;

async fn sse() -> impl Responder {
    // Create a stream of events
    let mut interval = interval(Duration::from_secs(1));
    let event_stream = async_stream::stream! {
        let mut count = 0;
        loop {
            interval.tick().await;
            count += 1;
            yield format!("data: Event number {}\n\n", count);
        }
    };

    // Convert stream to an SSE response
    HttpResponse::Ok()
        .content_type("text/event-stream")
        .streaming(event_stream)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| {
        App::new()
            .route("/events", web::get().to(sse))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}
```


## Project Structure

## Compiling the Project

## Running the Project
