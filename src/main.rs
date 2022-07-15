use clokwerk::{AsyncScheduler, TimeUnits};
use serde_json::Value;
use std::collections::HashMap;
use std::time::Duration;

async fn periodic_func() -> () {
    println!("Periodic task")
}

async fn test_req() -> () {
    let resp = req("http://ip-api.com/json/".to_string()).await;
    println!("{:#?}", resp);
}

async fn req(url: String) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
    let resp = reqwest::get(url)
        .await?
        .json::<HashMap<String, Value>>()
        .await?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Scheduler starts...");
    let mut scheduler = AsyncScheduler::new();
    scheduler.every(2.seconds()).run(periodic_func);
    scheduler.every(5.seconds()).run(test_req);
    loop {
        scheduler.run_pending().await;
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
