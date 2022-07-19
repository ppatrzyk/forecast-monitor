mod request;
mod kafka;

use chrono::prelude::*;
use config::Config;
use futures::future::join_all;
use itertools::izip;
use serde::{Serialize, Deserialize};

pub enum Source {
    OpenMeteo,
    Tomorrow,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct Forecast {
    source: String,
    forecast_time: String,
    weather_time: String,
    temperature: f64,
    precipitation: f64,
}

fn get_time() -> String {
    let time_utc: DateTime<Utc> = Utc::now();
    time_utc.to_rfc3339_opts(SecondsFormat::Millis, true)
}

pub async fn process(source: Source, config: Config) -> () {
    println!("process triggered");
    let messages:Vec<Forecast> = match source {
        Source::OpenMeteo => open_meteo(config).await,
        Source::Tomorrow => tomorrow(config).await,
    };
    println!("messages received");
    join_all(messages.into_iter().map(|msg| kafka::send(msg))).await;
    println!("messages sent");
}

async fn open_meteo(config: Config) -> Vec<Forecast> {
    let forecast_time: String = get_time();
    // https://api.open-meteo.com/v1/forecast?current_weather=true&timezone=UTC&latitude=51.11&longitude=17.03&hourly=temperature_2m,rain,showers
    let url = "https://api.open-meteo.com/v1/forecast/?hourly=temperature_2m,rain,showers".to_string();
    let query_params = [
        ("current_weather", "true"),
        ("timezone", "UTC"),
        ("latitude", &config.get_string("latitude").unwrap()),
        ("longitude", &config.get_string("longitude").unwrap()),
    ];
    let resp = request::req(url, &query_params).await;
    match resp {
        Err(_e) => {
            println!("OpenMeteo failed");
            vec![]
        }
        Ok(json) => {
            let mut forecasts: Vec<Forecast> = Vec::new();
            let times = json["hourly"]["time"].as_array().unwrap();
            let temps = json["hourly"]["temperature_2m"].as_array().unwrap();
            let precips = json["hourly"]["rain"].as_array().unwrap();
            for (time, temp, precip) in izip!(times, temps, precips) {
                let forecast = Forecast {
                    source: "open-meteo.com".to_string(),
                    forecast_time: forecast_time.to_owned(),
                    weather_time: time.to_string(),
                    temperature: temp.as_f64().unwrap(),
                    precipitation: precip.as_f64().unwrap(),
                };
                forecasts.push(forecast);
            }
            forecasts
        }
    }
}

async fn tomorrow(config: Config) -> Vec<Forecast> {
    let forecast_time: String = get_time();
    // https://api.tomorrow.io/v4/timelines?location=51.11,17.03&fields=temperature,precipitationIntensity&timesteps=1h&units=metric&timezone=UTC&apikey=
    let url = "https://api.tomorrow.io/v4/timelines".to_string();
    let query_params = [
        ("fields", "temperature,precipitationIntensity"),
        ("timezone", "UTC"),
        ("timesteps", "1h"),
        ("units", "metric"),
        ("apikey", &config.get_string("tomorrow").unwrap()),
        ("latlon", &[config.get_string("latitude").unwrap().as_str(), config.get_string("longitude").unwrap().as_str()].join(",")),
    ];
    let resp = request::req(url, &query_params).await;
    println!("{:?}", resp);
    match resp {
        Err(_e) => {
            println!("Tomorrow.io failed");
            vec![]
        }
        Ok(json) => {
            let mut forecasts: Vec<Forecast> = Vec::new();
            let entries = json["data"]["timelines"][0]["intervals"].as_array().unwrap();
            for entry in entries {
                let forecast = Forecast {
                    source: "tomorrow.io".to_string(),
                    forecast_time: forecast_time.to_owned(),
                    weather_time: entry["startTime"].to_string(),
                    temperature: entry["values"]["temperature"].as_f64().unwrap(),
                    precipitation: entry["values"]["precipitationIntensity"].as_f64().unwrap(),
                };
                println!("{:?}", forecast);
                forecasts.push(forecast);
            }
            forecasts
        }
    }
}
