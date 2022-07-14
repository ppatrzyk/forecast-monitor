use clokwerk::{Scheduler, TimeUnits};
use std::thread;
use std::time::Duration;

fn periodic_func() {
    println!("Periodic task")
}

fn main() {
    println!("Scheduler starts...");
    let mut scheduler = Scheduler::new();
    scheduler.every(2.seconds()).run(periodic_func);
    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(100));
    }
}
