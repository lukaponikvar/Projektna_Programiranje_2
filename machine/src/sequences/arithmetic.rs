use super::models::Sequence;
use crate::structs::range::Range;

/// Arithmetic sequence which takes two parameters: a start and a step.
pub struct Arithmetic {
    pub start: f64,
    pub step: f64,
}

impl Arithmetic {
    /// Creates a new `arithmetic` sequence.
    pub fn new(start: f64, step: f64) -> Box<Arithmetic> {
        Box::new(Arithmetic { start, step })
    }

    /// Returns `k_th` element of a given `arithmetic` sequence.
    pub fn k_th(&self, k: u64) -> f64 {
        self.start + (k as f64) * self.step
    }
}

impl Sequence<f64> for Arithmetic {
    fn range(&self, range: &Range) -> Vec<f64> {
        let mut result = Vec::new();
        let mut index = range.from;
        while index < range.to {
            result.push(self.k_th(index as u64));
            index += range.step;
        }
        result
    }
}
