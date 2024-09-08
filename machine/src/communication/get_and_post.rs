///Funkcija na dani url naslov pošlje `GET` zahtevo.
pub async fn send_get(url: &String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let result = client.get(url).send().await?.text().await?;
    return Ok(result);
}

///Funkcija na dani url naslov pošlje `POST` zahtevo.
pub async fn send_post(url: &String, body: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let result = client.post(url).body(body).send().await?.text().await?;
    return Ok(result);
}
