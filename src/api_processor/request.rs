use serde_json::Value;
use std::collections::HashMap;

pub async fn req(url: String, query_params: &[(&str, &str)]) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp = client.get(url)
        .query(query_params)
        .send()
        .await?
        .json::<HashMap<String, Value>>()
        .await?;
    Ok(resp)
}
