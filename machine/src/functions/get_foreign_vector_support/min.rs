use crate::{
    functions::{get_foreign_vector::get_foreign_vector, size::size},
    structs::{
        custom_error::CustomError,
        project::Project,
        range::Range,
        sequences::{SequenceInfo, SequenceSyntax},
    },
};
use futures::future::join_all;

/// Returns vector of `min` sequence terms in the given `range`.
///
/// ## Errors
/// In case of an error is no sequence that matches the requested sequence, the "Invalid input format" error is reported
/// with additional info about sequence that caused an error.
pub async fn min(
    syn: &SequenceSyntax,
    range: &Range,
    users: &Vec<Project>,
    all_sequences: &Vec<Vec<SequenceInfo>>,
) -> Result<Vec<f64>, CustomError> {
    let mut sequences_as_futures = Vec::new();
    for seq in &syn.sequences {
        sequences_as_futures.push(get_foreign_vector(&*seq, &range, users, all_sequences));
    }
    let mut sequences = Vec::new();
    for seq in join_all(sequences_as_futures).await {
        let vector = match seq {
            Ok(s) => s,
            Err(e) => return Err(CustomError::new(e.to_string())),
        };
        sequences.push(vector);
    }
    let size: usize = size(range);
    if sequences.len() == 0 {
        Ok(vec![0.0; size])
    } else {
        let mut result = sequences[0].clone();
        for vector in &sequences {
            for index in 0..size {
                if result[index] > vector[index] {
                    result[index] = vector[index]
                } else {
                    continue;
                }
            }
        }
        Ok(result)
    }
}
