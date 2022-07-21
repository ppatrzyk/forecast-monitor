mod api_processor;

use clokwerk::{AsyncScheduler, TimeUnits};
use config::Config;
use std::time::Duration;

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    let config = Config::builder()
        .add_source(config::File::with_name("config"))
        .build()?;
    println!("Scheduler starts...");
    let mut scheduler = AsyncScheduler::new();
    let config_meteo = config.clone();
    scheduler.every(5.minutes()).run(move || api_processor::process(api_processor::Source::OpenMeteo, config_meteo.clone()));
    let config_tomorrow = config.clone();
    scheduler.every(5.minutes()).run(move || api_processor::process(api_processor::Source::Tomorrow, config_tomorrow.clone()));
    let config_weatherapi = config.clone();
    scheduler.every(5.minutes()).run(move || api_processor::process(api_processor::Source::WeatherApi, config_weatherapi.clone()));
    loop {
        tokio::spawn(scheduler.run_pending());
        tokio::time::sleep(Duration::from_millis(100)).await;
    }
}
