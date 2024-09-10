use crate::{
    sequences::{constant::Constant, models::Sequence},
    structs::{custom_error::CustomError, range::Range, sequences::SequenceSyntax},
};

pub fn constant(syn: &SequenceSyntax, range: &Range) -> Result<Vec<f64>, CustomError> {
    Ok(Constant::new(syn.parameters[0]).range(&range))
}
