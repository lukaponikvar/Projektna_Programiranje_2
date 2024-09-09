use cosmwasm_std::testing::mock_env;
use futures::future::join_all;
use nois::{randomness_simulator, shuffle};

use crate::communication::find_owners::find_owners;
use crate::communication::request_vector::request_vector;
use crate::sequences::arithmetic::Arithmetic;
use crate::sequences::constant::Constant;
use crate::sequences::geometric::Geometric;
use crate::sequences::models::Sequence;
use crate::structs::custom_error::CustomError;
use crate::structs::project::Project;
use crate::structs::range::Range;
use crate::structs::sequences::{SequenceInfo, SequenceSyntax};
use async_recursion::async_recursion;

#[async_recursion]
pub async fn get_foreign_vector(
    syn: &SequenceSyntax,
    range: &Range,
    users: &Vec<Project>,
    all_sequences: &Vec<Vec<SequenceInfo>>,
) -> Result<Vec<f64>, CustomError> {
    let sequence: Vec<f64> = match &(syn).name {
        s if s == &"Constant".to_string() => Constant::new(syn.parameters[0]).range(&range),
        s if s == &"Arithmetic".to_string() => {
            Arithmetic::new(syn.parameters[0], syn.parameters[1]).range(&range)
        }
        s if s == &"Geometric".to_string() => {
            Geometric::new(syn.parameters[0], syn.parameters[1]).range(&range)
        }
        s if s == &"Sum".to_string() => {
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
                vec![0.0; size]
            } else {
                let mut result = vec![0.0; size];
                for vector in &sequences {
                    for index in 0..size {
                        result[index] += vector[index];
                    }
                }
                result
            }
        }
        s if s == &"Product".to_string() => {
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
                vec![1.0; size]
            } else {
                let mut result = vec![1.0; size];
                for vector in &sequences {
                    for index in 0..size {
                        result[index] *= vector[index];
                    }
                }
                result
            }
        }
        s if s == &"Drop".to_string() => {
            return get_foreign_vector(
                &*(syn.sequences[0]),
                &Range {
                    from: range.from + syn.parameters[0] as u64,
                    to: range.to + syn.parameters[0] as u64,
                    step: range.step,
                },
                users,
                all_sequences,
            )
            .await
        }
        _ => {
            let owners = find_owners(syn, users, all_sequences).await;
            let random_owners = shuffle(randomness_simulator(&mock_env()), owners); #TODO:
            for owner in random_owners.into_iter() {
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
    };
    Ok(sequence)
}
