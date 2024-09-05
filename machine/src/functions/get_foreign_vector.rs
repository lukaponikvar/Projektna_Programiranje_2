use core::panic;
use cosmwasm_std::testing::mock_env;
use nois::{randomness_simulator, shuffle};

use crate::communication::find_owners::find_owners;
use crate::communication::request_vector::request_vector;
use crate::sequences::arithmetic::Arithmetic;
use crate::sequences::constant::Constant;
use crate::sequences::geometric::Geometric;
use crate::sequences::models::Sequence;
use crate::structs::project::Project;
use crate::structs::range::Range;
use crate::structs::sequences::{SequenceInfo, SequenceSyntax};
use async_recursion::async_recursion;

#[async_recursion]
pub async fn get_foreign_vector(
    syn: SequenceSyntax,
    range: &Range,
    users: Vec<Project>,
    all_sequences: Vec<Vec<SequenceInfo>>,
) -> Vec<f64> {
    let sequence: Vec<f64> = match (syn).name.clone() {
        s if s == "Constant".to_string() => Constant::new(syn.parameters[0]).range(&range),
        s if s == "Arithmetic".to_string() => {
            Arithmetic::new(syn.parameters[0], syn.parameters[1]).range(&range)
        }
        s if s == "Geometric".to_string() => {
            Geometric::new(syn.parameters[0], syn.parameters[1]).range(&range)
        }
        s if s == "Sum".to_string() => {
            let mut sequences = Vec::new();
            for seq in syn.sequences {
                sequences.push(
                    get_foreign_vector(
                        *(seq.clone()),
                        &range,
                        users.clone(),
                        all_sequences.clone(),
                    )
                    .await,
                );
            }
            let size: usize = (range.to - range.from + 1) as usize;
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
        s if s == "Product".to_string() => {
            let mut sequences = Vec::new();
            for seq in syn.sequences {
                sequences.push(
                    get_foreign_vector(
                        *(seq.clone()),
                        &range,
                        users.clone(),
                        all_sequences.clone(),
                    )
                    .await,
                );
            }
            let size: usize = (range.to - range.from + 1) as usize;
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
        s if s == "Drop".to_string() => {
            get_foreign_vector(
                *(syn.sequences[0]).clone(),
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
            let owners = find_owners(syn.clone(), users, all_sequences).await;
            let random_owners = shuffle(randomness_simulator(&mock_env()), owners);
            for owner in random_owners.into_iter() {
                let possible_vector = request_vector(range, syn.clone(), owner).await;
                match possible_vector {
                    Some(s) => return s,
                    None => continue,
                }
            }
            panic!("Nobody talks to us :(")
        }
    };
    sequence
}

//TODO: Nehaj klonirat in začni uporabljati lifetime
