use clokwerk::{AsyncScheduler, TimeUnits};
use std::collections::HashMap;
use std::time::Duration;

async fn periodic_func() -> () {
    println!("Periodic task")
}

// async fn req() -> Result<(), Box<dyn std::error::Error>> {
async fn req() -> () {
    let resp = reqwest::get("https://httpbin.org/ip")
        .await.unwrap()
        .json::<HashMap<String, String>>()
        .await.unwrap();
    println!("{:#?}", resp);
    // Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Scheduler starts...");
    let mut scheduler = AsyncScheduler::new();
    scheduler.every(2.seconds()).run(periodic_func);
    scheduler.every(5.seconds()).run(req);
    loop {
        scheduler.run_pending().await;
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
