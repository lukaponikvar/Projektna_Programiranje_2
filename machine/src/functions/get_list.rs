use std::error::Error;

use crate::structs::range::Range;
use crate::structs::sequences::{SequenceRequest, SequenceSyntax};

async fn send_post(url: String, body: String) -> Result<String, reqwest::Error> {
    let client = reqwest::Client::new();
    let res = client.post(url).body(body).send().await?.text().await?;
    return Ok(res);
}

// TODO: Send_post v svoj file

pub async fn get_list(seq: SequenceSyntax, range: Range) -> Result<Vec<f64>, Box<dyn Error>> {
    // let users = send_get("http://127.0.0.1:7878/project".to_string()).await?;
    // for user in
    let parameters = seq.parameters.clone();
    let body: SequenceRequest = SequenceRequest {
        range,
        parameters,
        sequences: seq.sequences,
    };
    let url: String = format!("http://127.0.0.1:12345/sequence/Arithmetic");

    // let ours = sequences();
    // let sequence: Option<&SequenceInfo> = ours.iter().find(|&x| (x.name) == seq.name);
    // assert!(sequence.unwrap().parameters == seq.parameters.len() as u64);
    let response = send_post(url, serde_json::to_string(&body).unwrap()).await?;
    println!("{:#?}", response);
    let parsed: Vec<f64> = serde_json::from_str(&response).unwrap();
    println!("{:#?}", parsed);
    Ok(parsed)
}
