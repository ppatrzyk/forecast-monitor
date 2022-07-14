use clokwerk::{Scheduler, TimeUnits};
use std::thread;
use std::time::Duration;

fn main() {
    println!("Scheduler starts...");
    let mut scheduler = Scheduler::new();
    scheduler.every(10.seconds()).run(|| println!("Periodic task"));
    // let thread_handle = scheduler.watch_thread(Duration::from_millis(100));
    loop {
        scheduler.run_pending();
        thread::sleep(Duration::from_millis(100));
    }
}
