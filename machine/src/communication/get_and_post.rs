/// Sends a `GET` request to the given `url` address.
pub async fn send_get(url: &String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let result = client.get(url).send().await?.text().await?;
    return Ok(result);
}

/// Sends a `POST` request to the given `url` address.
pub async fn send_post(url: &String, body: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let result = client.post(url).body(body).send().await?.text().await?;
    return Ok(result);
}
