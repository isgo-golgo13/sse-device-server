use futures::StreamExt;
use reqwest::Client;
use std::fs::{OpenOptions};
use std::io::Write;
use tokio::time::{interval, Duration};

#[tokio::main]
async fn main() -> Result<(), reqwest::Error> {
    let client = Client::new();
    let res = client.get("http://127.0.0.1:8080/events")
        .send()
        .await?
        .bytes_stream();

    let mut file = OpenOptions::new()
        .create(true)
        .append(true)
        .open("device_upgrades.json")
        .expect("Unable to open file");

    res.for_each(|chunk| async {
        match chunk {
            Ok(data) => {
                let upgrade_data = String::from_utf8_lossy(&data);
                writeln!(file, "{}", upgrade_data).expect("Unable to write data");
            },
            Err(e) => eprintln!("Error receiving SSE data: {}", e),
        }
    }).await;

    Ok(())
}
