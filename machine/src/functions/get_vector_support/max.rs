use crate::{
    functions::get_vector::get_vector,
    structs::{custom_error::CustomError, range::Range, sequences::SequenceSyntax},
};

/// Returns vector of `max` sequence terms in the given `range`.
pub fn max(syn: &SequenceSyntax, range: &Range) -> Result<Vec<f64>, CustomError> {
    let mut sequences = Vec::new();
    for seq in &syn.sequences {
        match get_vector(&*seq, &range) {
            Ok(s) => sequences.push(s),
            Err(e) => return Err(CustomError::new(e.to_string())),
        }
    }
    let size: usize = (range.to - range.from) as usize;
    if sequences.len() == 0 {
        Ok(vec![0.0; size])
    } else {
        let mut result = sequences[0].clone();
        for vector in &sequences {
            for index in 0..size {
                if result[index] < vector[index] {
                    result[index] = vector[index]
                } else {
                    continue;
                }
            }
        }
        Ok(result)
    }
}
