use crate::{
    sequences::{arithmetic::Arithmetic, models::Sequence},
    structs::{custom_error::CustomError, range::Range, sequences::SequenceSyntax},
};

pub fn arithmetic(syn: &SequenceSyntax, range: &Range) -> Result<Vec<f64>, CustomError> {
    Ok(Arithmetic::new(syn.parameters[0], syn.parameters[1]).range(&range))
}
