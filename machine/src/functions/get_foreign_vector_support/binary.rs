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
pub async fn binary(
    syn: &SequenceSyntax,
    range: &Range,
    users: &Vec<Project>,
    all_sequences: &Vec<Vec<SequenceInfo>>,
) -> Result<Vec<f64>, CustomError> {
    match get_foreign_vector(&*(syn.sequences[0]), range, users, all_sequences).await {
        Ok(v) => Ok(v
            .iter()
            .map(|x| {
                format!("{:b}", *x as i64)
                    .parse()
                    .expect("This can't happen!")
            })
            .collect()),
        Err(e) => Err(CustomError::new(e.to_string())),
    }
}
