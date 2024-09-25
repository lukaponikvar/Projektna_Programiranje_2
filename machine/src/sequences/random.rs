use super::models::Sequence;
use crate::structs::range::Range;
use rand::Rng;

/// A sequence that takes two arguments: a and b, and generates
/// random values in a specified range [min(a, b), max(a,b)).
/// If the range is empty it returns all a's.
pub struct Random {
    pub min: f64,
    pub max: f64,
}

impl Random {
    /// Creates a new `random` sequence.
    pub fn new(min: f64, max: f64) -> Box<Random> {
        Box::new(Random { min, max })
    }
}

impl Sequence<f64> for Random {
    fn range(&self, range: &Range) -> Vec<f64> {
        let size: usize = (range.to - range.from) as usize;
        let (mut min, mut max) = (self.min, self.max);
        if max < min {
            (min, max) = (self.max, self.min);
        } else if max == min {
            return vec![max; size];
        }
        let mut result = vec![];
        let mut rng = rand::thread_rng();
        for _ in 0..size {
            result.push(rng.gen_range(min..max));
        }
        result
    }
}
