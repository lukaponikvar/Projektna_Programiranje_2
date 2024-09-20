use crate::{
    sequences::{geometric::Geometric, models::Sequence},
    structs::{custom_error::CustomError, range::Range, sequences::SequenceSyntax},
};

/// Returns vector of `geometric` sequence terms in the given `range`.
pub fn geometric(syn: &SequenceSyntax, range: &Range) -> Result<Vec<f64>, CustomError> {
    Ok(Geometric::new(syn.parameters[0], syn.parameters[1]).range(&range))
}
