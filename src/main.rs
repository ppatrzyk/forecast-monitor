use clokwerk::{AsyncScheduler, TimeUnits};
use std::time::Duration;

async fn periodic_func() {
    println!("Periodic task")
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("Scheduler starts...");
    let mut scheduler = AsyncScheduler::new();
    scheduler.every(2.seconds()).run(periodic_func);
    loop {
        scheduler.run_pending().await;
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
