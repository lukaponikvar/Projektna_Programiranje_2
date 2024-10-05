use super::models::Sequence;
use crate::structs::range::Range;

/// A sequence `(b_n)` that takes a single sequence `(a_n)` and returns a sequence where `b_i = (a_1 + a_2 + ... + a_i)/i`.
pub struct Averages {
    pub sequence: Box<dyn Sequence<f64>>,
}

impl Averages {
    /// Creates a new `averages` sequence.
    pub fn new(sequence: Box<dyn Sequence<f64>>) -> Box<Averages> {
        Box::new(Averages { sequence })
    }
}

impl Sequence<f64> for Averages {
    fn range(&self, range: &Range) -> Vec<f64> {
        let long_vector = self.sequence.range(&Range {
            from: 0,
            to: range.to,
            step: 1,
            });
        let mut middle_vector = Vec::new();
        let mut sum = 0.0;
        let mut i = 1.0;
        for float in long_vector {
            sum += float;
            middle_vector.push(sum/i);
            i += 1.0
        } 
        let mut final_vector = Vec::new();
        let a:u64 = ((range.to as f64 - range.from as f64) / range.step as f64).ceil() as u64;
        for i in 0..a {
            final_vector.push(middle_vector[(range.from + i*range.step) as usize]);
        }
        final_vector
    }
}
