use super::get_vector_support::{
    arithmetic::arithmetic, average::average, constant::constant, drop::drop, fibonacci::fibonacci,
    geometric::geometric, linear_combination::linear_combination, max::max, min::min,
    product::product, sum::sum,
};
use crate::structs::{custom_error::CustomError, range::Range, sequences::SequenceSyntax};

/// Returns vector of searched sequence terms from our server in the given `range`.
/// 
/// ## Errors
/// In case there is no sequence that matches the requested sequence `syn`, the "Invalid input format" error is reported
/// with additional info about requested sequence.
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
        s if s == &"Fibonacci".to_string() => fibonacci(syn, range),
        s if s == &"LinearCombination".to_string() => linear_combination(syn, range),
        s if s == &"Average".to_string() => average(syn, range),
        _ => Err(CustomError::new(format!(
            "Invalid input format\n Sequence: {}, with {} parameters and {} sequences not found.",
            syn.name,
            syn.parameters.len(),
            syn.sequences.len()
        ))), //TODO: Popravi na smiselno sporočilo
    }
}
