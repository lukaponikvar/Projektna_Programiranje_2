use super::get_vector_support::{
    arithmetic::arithmetic, constant::constant, drop::drop, geometric::geometric, max::max,
    min::min, product::product, sum::sum,
};
use crate::structs::{custom_error::CustomError, range::Range, sequences::SequenceSyntax};

pub fn get_vector(syn: &SequenceSyntax, range: &Range) -> Result<Vec<f64>, CustomError> {
    match &(syn).name {
        s if s == &"Constant".to_string() => constant(syn, range),
        s if s == &"Arithmetic".to_string() => arithmetic(syn, range),
        s if s == &"Geometric".to_string() => geometric(syn, range),
        s if s == &"Sum".to_string() => sum(syn, range),
        s if s == &"Product".to_string() => product(syn, range),
        s if s == &"Drop".to_string() => drop(syn, range),
        s if s == &"Min".to_string() => min(syn, range),
        s if s == &"Max".to_string() => max(syn, range),
        // s if s == &"Drop".to_string() =>
        // s if s == &"Drop".to_string() =>
        // s if s == &"Drop".to_string() =>
        _ => Err(CustomError::new("500".to_string())), //TODO: Popravi na smiselno sporočilo
    }
}
