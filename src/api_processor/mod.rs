mod request;

pub enum Source {
    OpenMeteo,
}

pub async fn test_req(url: String) -> () {
    let resp = request::req(url, &[]).await;
    println!("{:#?}", resp);
}

// TODO parse_func should return some common data structure
pub async fn process(source: Source) -> () {
    // take processing func as arg
    // write to kafka

    let resp = match source {
        Source::OpenMeteo => open_meteo().await
    };
    println!("{:#?}", resp);
}

async fn open_meteo() -> String {
    let url = "https://api.open-meteo.com/v1/forecast/?hourly=temperature_2m,rain,showers".to_string();
    let query_params = [
        ("current_weather", "true"),
        ("timezone", "UTC"),
        ("latitude", "51.11"),
        ("longitude", "17.03"),
    ];
    let resp = request::req(url, &query_params).await;
    // TODO pattern match if it's ok here, return common anyway
    println!("{:#?}", resp);
    "TODO common structure".to_string()
    // https://api.open-meteo.com/v1/forecast?current_weather=true&timezone=UTC&latitude=51.11&longitude=17.03&hourly=temperature_2m,rain,showers
}
