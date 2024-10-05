use super::models::Sequence;
use crate::{functions::size::size, structs::range::Range};

/// A sequence that takes two sequences and term by term calculates their average.
pub struct Average {
    pub sequences: Vec<Box<dyn Sequence<f64>>>,
}

impl Average {
    /// Creates a new `average` sequence.
    pub fn new(sequences: Vec<Box<dyn Sequence<f64>>>) -> Box<Average> {
        Box::new(Average { sequences })
    }
}

impl Sequence<f64> for Average {
    fn range(&self, range: &Range) -> Vec<f64> {
        let size: usize = size(range);
        let number = self.sequences.len();
        if number == 0 {
            vec![0.0; size]
        } else {
            let mut vectors = Vec::new();
            for seq in &self.sequences {
                vectors.push(seq.range(&range))
            }
            let mut almost_result = vec![0.0; size];
            for vector in &vectors {
                for index in 0..size {
                    almost_result[index] += vector[index];
                }
            }
            let mut result = vec![];
            for term in almost_result {
                result.push(term / number as f64);
            }
            result
        }
    }
}
