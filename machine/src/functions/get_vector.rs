use core::panic;

use crate::sequences::arithmetic::Arithmetic;
use crate::sequences::constant::Constant;
use crate::sequences::geometric::Geometric;
use crate::sequences::models::Sequence;
use crate::structs::range::Range;
use crate::structs::sequences::SequenceSyntax;
use async_recursion::async_recursion;

#[async_recursion]
pub async fn get_range(syn: SequenceSyntax, range: &Range) -> Vec<f64> {
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
                sequences.push(get_range(*(seq.clone()), &range).await);
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
                sequences.push(get_range(*(seq.clone()), &range).await);
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
            get_range(
                *(syn.sequences[0]).clone(),
                &Range {
                    from: range.from + syn.parameters[0] as u64,
                    to: range.to + syn.parameters[0] as u64,
                    step: range.step,
                },
            )
            .await
        }
        _ => panic!("Ne vem, kaj se dogaja:("),
    };
    sequence
}
