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
        "Sequence: {}, with {} parameters and {} sequences not found.",
        syn.name,
        syn.parameters.len(),
        syn.sequences.len()
    )));
}
