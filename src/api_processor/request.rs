use serde_json::Value;

pub async fn req(url: String, query_params: &[(&str, &str)]) -> Result<Value, Box<dyn std::error::Error>> {
    let client = reqwest::Client::new();
    let resp = client.get(url)
        .query(query_params)
        .send()
        .await?
        .json::<Value>()
        .await?;
    Ok(resp)
}
