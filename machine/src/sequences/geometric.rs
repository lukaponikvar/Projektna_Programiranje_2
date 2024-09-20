use super::models::Sequence;
use crate::mathematical_functions::power::power;
use crate::structs::range::Range;

/// Geometric sequence with two parameters: a start and a quotient.
pub struct Geometric {
    pub start: f64,
    pub quotient: f64,
}

impl Geometric {
    /// Creates a new `geometric` sequence.
    pub fn new(start: f64, quotient: f64) -> Box<Geometric> {
        Box::new(Geometric { start, quotient })
    }

    /// Returns `k_th` element of a given `Fibonacci` sequence.
    pub fn k_th(&self, k: u64) -> f64 {
        self.start * power(self.quotient, k)
    }
}

impl Sequence<f64, dyn Send> for Geometric {
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
