use crate::{
    functions::get_foreign_vector::get_foreign_vector,
    structs::{
        custom_error::CustomError,
        project::Project,
        range::Range,
        sequences::{SequenceInfo, SequenceSyntax},
    },
};

/// Returns vector of `drop` sequence terms in the given `range`.
/// 
/// ## Errors
/// In case of an error is no sequence that matches the requested sequence, the "Invalid input format" error is reported
/// with additional info about sequence that caused an error.
pub async fn drop(
    syn: &SequenceSyntax,
    range: &Range,
    users: &Vec<Project>,
    all_sequences: &Vec<Vec<SequenceInfo>>,
) -> Result<Vec<f64>, CustomError> {
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
    .await;
}
