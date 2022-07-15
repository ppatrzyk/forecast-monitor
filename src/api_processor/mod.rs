mod request;
use std::future::Future;

pub async fn test_req(url: String) -> () {
    let resp = request::req(url, &[]).await;
    println!("{:#?}", resp);
}

// TODO parse_func should return some common data structure
pub async fn process(parse_func: fn() -> String) -> () {
    // take processing func as arg
    // write to kafka
    let resp = parse_func();
}

pub async fn open_meteo() -> String {
    let url = "https://api.open-meteo.com/v1/".to_string();
    let query_params = [
        ("current_weather", "true"),
        ("timezone", "UTC"),
        ("latitude", "51.11"),
        ("longitude", "17.03"),
        ("hourly", "temperature_2m,rain,showers"),
    ];
    let resp = request::req(url, &query_params).await;
    // TODO pattern match if it's ok here, return common anyway
    println!("{:#?}", resp);
    "TODO common structure".to_string()
    // https://api.open-meteo.com/v1/forecast?current_weather=true&timezone=UTC&latitude=51.11&longitude=17.03&hourly=temperature_2m,rain,showers
}
