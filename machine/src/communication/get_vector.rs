use super::find_owner::find_owner;
use super::get_and_post::send_post;
use crate::structs::project::Project;
use crate::structs::range::Range;
use crate::structs::sequences::{SequenceRequest, SequenceSyntax};

///Funkcija pridobi seznam členov zaporedja v odvisnosti od `range`.
pub async fn get_vector(range: Range, sequence: SequenceSyntax) -> Vec<f64> {
    let owner: Project = find_owner(sequence.clone()).await;
    let url = format!(
        "http://{}:{}/sequence/{}",
        owner.ip, owner.port, sequence.name
    );
    let body = SequenceRequest {
        range,
        parameters: sequence.parameters,
        sequences: sequence.sequences,
    };
    let body_as_string = match serde_json::to_string(&body) {
        Ok(b) => b,
        Err(e) => panic!("{}", e),
    };
    let reply = match send_post(url, body_as_string).await {
        Ok(r) => r,
        Err(e) => panic!("{}", e),
    };
    let result = match serde_json::from_str(&reply) {
        Ok(b) => b,
        Err(e) => panic!("{}", e),
    };
    result
}
