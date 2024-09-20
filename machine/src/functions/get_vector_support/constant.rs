use crate::{
    sequences::{constant::Constant, models::Sequence},
    structs::{custom_error::CustomError, range::Range, sequences::SequenceSyntax},
};

/// Returns vector of `constant` sequence terms in the given `range`.
pub fn constant(syn: &SequenceSyntax, range: &Range) -> Result<Vec<f64>, CustomError> {
    Ok(Constant::new(syn.parameters[0]).range(&range))
}
