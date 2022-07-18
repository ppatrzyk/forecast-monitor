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
    scheduler.every(1.seconds()).run(periodic_func);
    scheduler.every(5.seconds()).run(|| api_processor::process(api_processor::Source::OpenMeteo));
    loop {
        tokio::spawn(scheduler.run_pending());
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
