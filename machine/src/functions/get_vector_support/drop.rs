use crate::{
    functions::get_vector::get_vector,
    structs::{custom_error::CustomError, range::Range, sequences::SequenceSyntax},
};

pub fn drop(syn: &SequenceSyntax, range: &Range) -> Result<Vec<f64>, CustomError> {
    return get_vector(
        &*(syn.sequences[0]),
        &Range {
            from: range.from + syn.parameters[0] as u64,
            to: range.to + syn.parameters[0] as u64,
            step: range.step,
        },
    );
}
