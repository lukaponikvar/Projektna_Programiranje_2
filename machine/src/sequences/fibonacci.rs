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

    /// Returns `k_th` element of a given `Fibonacci` sequence.
    fn k_th(&self, k: u64) -> f64 {
        k_th(self.zeroth, self.first, k)
    }
}

impl Sequence<f64> for Fibonacci {
    fn range(&self, range: &Range) -> Vec<f64> {
        let mut result = vec![];
        for index in range.from..range.to {
            result.push(self.k_th(index));
        }
        result
    }
}

fn k_th(zeroth: f64, first: f64, k: u64) -> f64 {
    if k == 0 {
        return zeroth;
    } else if k == 1 {
        return first;
    } else {
        return k_th(first, first + zeroth, k - 1);
    }
}
