mod request;

pub async fn test_req(url: String) -> () {
    let resp = request::req(url).await;
    println!("{:#?}", resp);
}

// TODO parse_func should return some common data structure
pub async fn process(parse_func: fn(String) -> String) -> () {
    // take processing func as arg
    // write to kafka
}

async fn open_meteo() {
    // TODO
    // https://api.open-meteo.com/v1/forecast?current_weather=true&timezone=UTC&latitude=51.11&longitude=17.03&hourly=temperature_2m,rain,showers
}
