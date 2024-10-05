use super::models::Sequence;
use crate::{functions::size::size, structs::range::Range};

/// A sequence that takes two sequences and uses their minimum term by term.
pub struct Min {
    pub sequences: Vec<Box<dyn Sequence<f64>>>,
}

impl Min {
    /// Creates a new `min` sequence.
    pub fn new(sequences: Vec<Box<dyn Sequence<f64>>>) -> Box<Min> {
        Box::new(Min { sequences })
    }
}

impl Sequence<f64> for Min {
    fn range(&self, range: &Range) -> Vec<f64> {
        let size: usize = size(range);
        if self.sequences.len() == 0 {
            vec![0.0; size]
        } else {
            let mut vectors = Vec::new();
            for seq in &self.sequences[1..] {
                vectors.push(seq.range(&range))
            }
            let mut result = self.sequences[0].range(&range);
            for vector in &vectors {
                for index in 0..size {
                    if result[index] > vector[index] {
                        result[index] = vector[index]
                    } else {
                        continue;
                    };
                }
            }
            result
        }
    }
}
