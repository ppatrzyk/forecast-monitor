mod request;

pub enum Source {
    OpenMeteo,
}

#[derive(Debug)]
struct Forecast {
    forecast_time: String,
    weather_time: String,
    temperature: f64,
    precipitation: f64,
}

pub async fn test_req(url: String) -> () {
    let resp = request::req(url, &[]).await;
    println!("{:#?}", resp);
}

pub async fn process(source: Source) -> () {
    let resp:Vec<Forecast> = match source {
        Source::OpenMeteo => open_meteo().await
    };
    println!("{:#?}", resp);
    // TODO write to kafka
}

async fn open_meteo() -> Vec<Forecast> {
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
            let mut v: Vec<Forecast> = Vec::new();
            println!("{:#?}", map["hourly"]);
            let forecast = Forecast {
                forecast_time: "dummy".to_string(),
                weather_time: "dummy".to_string(),
                temperature: 0.0,
                precipitation: 0.0,
            };
            v.push(forecast);
            v
        }
    };
    forecasts
}
