use super::find_owner::find_owner;
use crate::structs::project::Project;
use crate::structs::range::Range;
use crate::structs::sequences::{SequenceRequest, SequenceSyntax};

use super::get_and_post::send_post;

pub async fn get_foreign_vectors(range: Range, sequence: SequenceSyntax) -> Vec<f64> {
    let owner: Project = find_owner(sequence.clone()).await;
    let url = format!("http://{}:{}/sequence/{}", owner.ip, owner.port, owner.name);
    let body = SequenceRequest {
        range,
        parameters: sequence.parameters,
        sequences: sequence.sequences,
    };
    let stringed_body = serde_json::to_string(&body);
    let true_body = match stringed_body {
        Ok(b) => b,
        Err(e) => panic!("{}", e),
    };
    let response = send_post(url, true_body).await;
    let result = match response {
        Ok(r) => serde_json::from_str(&r),
        Err(e) => panic!("{}", e),
    };
    let result_2 = match result {
        Ok(b) => b,
        Err(e) => panic!("{}", e),
    };
    result_2
}
