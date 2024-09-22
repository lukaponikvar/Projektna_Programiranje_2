use crate::{
    sequences::{arithmetic::Arithmetic, models::Sequence},
    structs::{custom_error::CustomError, range::Range, sequences::SequenceSyntax},
};

/// Returns vector of `arithmetic` sequence terms in the given `range`.
pub fn arithmetic(syn: &SequenceSyntax, range: &Range) -> Result<Vec<f64>, CustomError> {
    Ok(Arithmetic::new(syn.parameters[0], syn.parameters[1]).range(&range))
}
