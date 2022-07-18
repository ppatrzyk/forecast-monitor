use crate::api_processor::Forecast;
use rdkafka::ClientConfig;
use rdkafka::producer::{FutureProducer, FutureRecord};
use rdkafka::util::Timeout;

pub async fn send(msg: Forecast) -> () {
    let serialized_msg = serde_json::to_string(&msg).unwrap();
    let kafka_msg = FutureRecord::to("weather")
        .payload(&serialized_msg)
        .key(&msg.forecast_time);
    // TODO change to docker address later
    // TODO - this error not passed higher?
    let producer: &FutureProducer = &ClientConfig::new()
        .set("bootstrap.servers", "localhost:39092")
        .create()
        .expect("Producer creation error");
    let _delivery_status = producer.send(kafka_msg, Timeout::Never).await;
}
