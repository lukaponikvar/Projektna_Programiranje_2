use super::models::Sequence;
use crate::{functions::size::size, structs::range::Range};

/// Constant sequence with a single parameter: value.
pub struct Constant {
    pub value: f64,
}

impl Constant {
    /// Creates a new `constant` sequence.
    pub fn new(value: f64) -> Box<Constant> {
        Box::new(Constant { value })
    }
}

impl Sequence<f64> for Constant {
    fn range(&self, range: &Range) -> Vec<f64> {
        let result = vec![self.value; size(range)];
        result
    }
}
