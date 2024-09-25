use crate::{
    sequences::{models::Sequence, random::Random},
    structs::{custom_error::CustomError, range::Range, sequences::SequenceSyntax},
};

/// Returns vector of `geometric` sequence terms in the given `range`.
pub fn random(syn: &SequenceSyntax, range: &Range) -> Result<Vec<f64>, CustomError> {
    Ok(Random::new(syn.parameters[0], syn.parameters[1]).range(&range))
}
