use futures::future::join_all;

use crate::{
    functions::get_foreign_vector::get_foreign_vector,
    structs::{
        custom_error::CustomError,
        project::Project,
        range::Range,
        sequences::{SequenceInfo, SequenceSyntax},
    },
};

/// Returns vector of `linear_combination` sequence terms in the given `range`.
///
/// ## Errors
/// In case of an error is no sequence that matches the requested sequence, the "Invalid input format" error is reported
/// with additional info about sequence that caused an error.
pub async fn linear_combination(
    syn: &SequenceSyntax,
    range: &Range,
    users: &Vec<Project>,
    all_sequences: &Vec<Vec<SequenceInfo>>,
) -> Result<Vec<f64>, CustomError> {
    let mut sequences_as_futures = Vec::new();
    for seq in &syn.sequences {
        sequences_as_futures.push(get_foreign_vector(&*seq, &range, users, all_sequences));
    }
    let sequences = join_all(sequences_as_futures).await;
    let first_vector = match &sequences[0] {
        Ok(s) => s,
        Err(e) => return Err(CustomError::new(e.to_string())),
    };
    let second_vector = match &sequences[1] {
        Ok(s) => s,
        Err(e) => return Err(CustomError::new(e.to_string())),
    };

    let mut result = vec![0.0; first_vector.len()];
    for indeks in 0..first_vector.len() {
        result[indeks] += syn.parameters[0] * first_vector[indeks]
            + syn.parameters[1] * second_vector[indeks]
            + syn.parameters[2]
    }
    Ok(result)
}
