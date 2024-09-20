use super::get_and_post::send_post;
use crate::structs::{
    custom_error::CustomError,
    project::Project,
    range::Range,
    sequences::{SequenceRequest, SequenceSyntax},
};

/// Returns vector of searched sequence terms of the selected `owner` in the given `range`.
/// 
/// ## Errors
/// In case of errors, they are reported.
pub async fn request_vector(
    range: &Range,
    sequence: &SequenceSyntax,
    owner: &Project,
) -> Result<Vec<f64>, CustomError> {
    let url = format!(
        "http://{}:{}/sequence/{}",
        owner.ip, owner.port, sequence.name
    );
    let body = SequenceRequest {
        range: range.clone(),
        parameters: sequence.parameters.clone(),
        sequences: sequence.sequences.clone(),
    };
    let body_as_string = match serde_json::to_string(&body) {
        Ok(b) => b,
        Err(e) => return Err(CustomError::new(e.to_string())),
    };
    let reply = match send_post(&url, body_as_string).await {
        Ok(r) => r,
        Err(e) => return Err(CustomError::new(e.to_string())),
    };
    match serde_json::from_str(&reply) {
        Ok(b) => check_vector(range, b),
        Err(e) => Err(CustomError::new(e.to_string())),
    }
}

/// Checks if length of the given `vector` matches length of the requested `range`.
/// 
/// ## Errors
/// If the returned `vector` is not suitable for the requested `range`, 
/// the error is reported with additional information about lengths of vectors.
fn check_vector(range: &Range, vector: Vec<f64>) -> Result<Vec<f64>, CustomError> {
    if (range.to - range.from) as usize != vector.len() {
        return Err(CustomError::new(
            format!("Invalid return.\nVector length: {}, range length: {}.", 
            vector.len(), 
            range.to - range.from)
        ));
    } else {
        return Ok(vector);
    }
}
