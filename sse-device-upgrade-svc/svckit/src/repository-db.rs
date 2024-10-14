use crate::db::DbPool;
use diesel::prelude::*;
use redis::AsyncCommands;
use tokio::task;
use chrono::Utc;

pub async fn device_upgrade_detector(pool: DbPool) -> String {
    let mut conn = pool.get().expect("Could not get DB connection");
    let mut redis_conn = redis::Client::open("redis://127.0.0.1/").unwrap().get_async_connection().await.unwrap();

    let redis_key = "latest_upgrade";

    if let Ok(cached_upgrade) = redis_conn.get::<_, String>(redis_key).await {
        return cached_upgrade;
    }

    // Query latest upgrade from DB
    let result: String = "dummy_db_upgrade_row".to_string(); // Query DB here.

    // Update Redis cache
    let _: () = redis_conn.set(redis_key, &result).await.unwrap();

    result
}

pub async fn update_firmware(pool: DbPool) {
    task::spawn_blocking(move || {
        let conn = pool.get().expect("Failed to get DB connection");

        // Insert new row with updated firmware version and new upgrade date
        diesel::sql_query(
            "INSERT INTO device_upgrades (device_firmware_version, firmware_upgrade_date) VALUES (...);"
        )
        .execute(&conn)
        .expect("Insert failed");
    }).await.expect("Task panicked");
}
