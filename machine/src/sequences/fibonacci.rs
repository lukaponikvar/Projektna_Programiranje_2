use super::models::Sequence;
use crate::structs::range::Range;

/// Fibonacci sequence starting with given parameters `zeroth` and `first`.
pub struct Fibonacci {
    pub zeroth: f64,
    pub first: f64,
}

impl Fibonacci {
    /// Creates a new `Fibonacci` sequence.
    pub fn new(zeroth: f64, first: f64) -> Box<Fibonacci> {
        Box::new(Fibonacci { zeroth, first })
    }
}

impl Sequence<f64> for Fibonacci {
    fn range(&self, range: &Range) -> Vec<f64> {
        let mut first = self.first;
        let mut zeroth = self.zeroth;
        let mut result: Vec<f64> = Vec::new();
        if range.from == 0 && range.to > 0 {
            result.push(zeroth);
        }
        if range.from <= 1 && range.to > 1 && ((1 - range.from) % range.step == 0) {
            result.push(first);
        }
        let mut i = 2;
        while i < range.to {
            let t = zeroth + first;
            zeroth = first;
            first = t;
            if i >= range.from && ((i - range.from) % range.step == 0) {
                result.push(first);
            }
            i += 1;
        }
        return result;
    }
}
