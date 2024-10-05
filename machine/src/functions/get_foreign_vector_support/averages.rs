use crate::{
    functions::get_foreign_vector::get_foreign_vector,
    structs::{
        custom_error::CustomError,
        project::Project,
        range::Range,
        sequences::{SequenceInfo, SequenceSyntax},
    },
};

/// Returns vector of `averages` sequence terms in the given `range`.
/// 
/// ## Errors
/// In case of an error is no sequence that matches the requested sequence, the "Invalid input format" error is reported
/// with additional info about sequence that caused an error.
pub async fn averages(
    syn: &SequenceSyntax,
    range: &Range,
    users: &Vec<Project>,
    all_sequences: &Vec<Vec<SequenceInfo>>,
) -> Result<Vec<f64>, CustomError> {
    let a_range = &Range {
        from: 0,
        to: range.to,
        step: 1,
        };
    let sequence_as_future = get_foreign_vector(&*syn.sequences[0], a_range , users, all_sequences);
    let vector = match sequence_as_future.await {
        Ok(s) => s,
        Err(e) => return Err(CustomError::new(e.to_string())),
    };
    let mut middle_vector = Vec::new();
    let mut sum = 0.0;
    let mut i = 1.0;
    for float in vector {
        sum += float;
        middle_vector.push(sum/i);
        i += 1.0
    } 
    let mut final_vector = Vec::new();
    let a:u64 = ((range.to as f64 - range.from as f64) / range.step as f64).ceil() as u64;
    for i in 0..a {
        final_vector.push(middle_vector[(range.from + i*range.step) as usize]);
    }
    Ok(final_vector)
    }
