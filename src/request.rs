use reqwest::get;
use serde_json::Value;
use std::collections::HashMap;

pub async fn req(url: String) -> Result<HashMap<String, Value>, Box<dyn std::error::Error>> {
    let resp = get(url)
        .await?
        .json::<HashMap<String, Value>>()
        .await?;
    Ok(resp)
}
