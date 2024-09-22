use crate::{
    functions::get_vector::get_vector,
    structs::{custom_error::CustomError, range::Range, sequences::SequenceSyntax},
};

/// Returns vector of `average` sequence terms in the given `range`.
pub fn average(syn: &SequenceSyntax, range: &Range) -> Result<Vec<f64>, CustomError> {
    let mut sequences = Vec::new();
    for seq in &syn.sequences {
        match get_vector(&*seq, &range) {
            Ok(s) => sequences.push(s),
            Err(e) => return Err(CustomError::new(e.to_string())),
        }
    }
    let size: usize = (range.to - range.from) as usize;
    let number = sequences.len();
    if number == 0 {
        Ok(vec![0.0; size])
    } else {
        let mut almost_result = vec![0.0; size];
        for vector in &sequences {
            for index in 0..size {
                almost_result[index] += vector[index];
            }
        }
        let mut result = vec![];
        for term in almost_result {
            result.push(term / number as f64);
        }
        Ok(result)
    }
}
