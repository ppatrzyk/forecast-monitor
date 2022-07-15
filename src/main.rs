mod api_processor;

use clokwerk::{AsyncScheduler, TimeUnits};
use std::time::Duration;

async fn periodic_func() -> () {
    println!("Periodic task")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Scheduler starts...");
    let mut scheduler = AsyncScheduler::new();
    scheduler.every(2.seconds()).run(periodic_func);
    scheduler.every(5.seconds()).run(|| api_processor::test_req("http://ip-api.com/json/".to_string()));
    loop {
        scheduler.run_pending().await;
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
