mod request;
mod kafka;

use chrono::prelude::*;
use futures::future::join_all;
use itertools::izip;
use serde::{Serialize, Deserialize};

pub enum Source {
    OpenMeteo,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Forecast {
    source: String,
    forecast_time: String,
    weather_time: String,
    temperature: f64,
    precipitation: f64,
}

pub async fn process(source: Source) -> () {
    println!("process triggered");
    let messages:Vec<Forecast> = match source {
        Source::OpenMeteo => open_meteo().await
    };
    println!("messages received");
    join_all(messages.into_iter().map(|msg| kafka::send(msg))).await;
    println!("messages sent");
}

async fn open_meteo() -> Vec<Forecast> {
    let forecast_time_dt: DateTime<Utc> = Utc::now();
    let forecast_time_str: String = forecast_time_dt.to_rfc3339_opts(SecondsFormat::Millis, true);
    // https://api.open-meteo.com/v1/forecast?current_weather=true&timezone=UTC&latitude=51.11&longitude=17.03&hourly=temperature_2m,rain,showers
    let url = "https://api.open-meteo.com/v1/forecast/?hourly=temperature_2m,rain,showers".to_string();
    let query_params = [
        ("current_weather", "true"),
        ("timezone", "UTC"),
        ("latitude", "51.11"),
        ("longitude", "17.03"),
    ];
    let resp = request::req(url, &query_params).await;

    let forecasts = match resp {
        Err(_e) => {
            println!("OpenMeteo failed");
            vec![]
        }
        Ok(map) => {
            let mut forecasts: Vec<Forecast> = Vec::new();
            let times = map["hourly"]["time"].as_array().unwrap();
            let temps = map["hourly"]["temperature_2m"].as_array().unwrap();
            let precips = map["hourly"]["rain"].as_array().unwrap();
            for (time, temp, precip) in izip!(times, temps, precips) {
                let forecast = Forecast {
                    source: "open-meteo.com".to_string(),
                    forecast_time: forecast_time_str.to_owned(),
                    weather_time: time.to_string(),
                    temperature: temp.as_f64().unwrap(),
                    precipitation: precip.as_f64().unwrap(),
                };
                // println!("{:#?}", forecast);
                forecasts.push(forecast);
            }
            forecasts
        }
    };
    forecasts
}
