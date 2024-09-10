use crate::{
    functions::get_foreign_vector::get_foreign_vector,
    structs::{
        custom_error::CustomError,
        project::Project,
        range::Range,
        sequences::{SequenceInfo, SequenceSyntax},
    },
};

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
