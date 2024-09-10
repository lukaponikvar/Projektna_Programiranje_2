use crate::{
    functions::get_foreign_vector::get_foreign_vector,
    structs::{
        custom_error::CustomError,
        project::Project,
        range::Range,
        sequences::{SequenceInfo, SequenceSyntax},
    },
};
use futures::future::join_all;

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
    let size: usize = (range.to - range.from) as usize;
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
