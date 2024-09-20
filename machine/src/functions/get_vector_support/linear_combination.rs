use crate::{
    functions::get_vector::get_vector,
    structs::{custom_error::CustomError, range::Range, sequences::SequenceSyntax},
};

/// Returns vector of `linear_combination` sequence terms in the given `range`.
pub fn linear_combination(syn: &SequenceSyntax, range: &Range) -> Result<Vec<f64>, CustomError> {
    let first_vector = match get_vector(&syn.sequences[0], range) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let second_vector = match get_vector(&syn.sequences[1], range) {
        Ok(v) => v,
        Err(e) => return Err(e),
    };
    let mut result = vec![0.0; first_vector.len()];
    for indeks in 0..first_vector.len() {
        result[indeks] +=
            syn.parameters[0] * first_vector[indeks] + syn.parameters[1] * second_vector[indeks]
    }
    Ok(result)
}
