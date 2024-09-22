use crate::{
    communication::{find_owners::find_owners, request_vector::request_vector},
    structs::{
        custom_error::CustomError,
        project::Project,
        range::Range,
        sequences::{SequenceInfo, SequenceSyntax},
    },
};
use rand::{seq::SliceRandom, thread_rng};


/// Returns vector of searched sequence terms from its random owner (in case there is more of them) in the given `range`.
/// 
/// ## Errors
/// In case there is no sequence that matches the requested sequence `syn`, the "Invalid input format" error is reported
/// with additional info about requested sequence.
pub async fn other(
    syn: &SequenceSyntax,
    range: &Range,
    users: &Vec<Project>,
    all_sequences: &Vec<Vec<SequenceInfo>>,
) -> Result<Vec<f64>, CustomError> {
    let mut owners = find_owners(syn, users, all_sequences).await;
    owners.shuffle(&mut thread_rng());
    for owner in owners.into_iter() {
        let possible_vector = request_vector(&range, syn, &owner).await;
        match possible_vector {
            Ok(s) => return Ok(s),
            Err(_) => continue,
        }
    }
    return Err(CustomError::new(format!(
        "Invalid input format\n Sequence: {}, with {} parameters and {} sequences not found.",
        syn.name,
        syn.parameters.len(),
        syn.sequences.len()
    )));
}
