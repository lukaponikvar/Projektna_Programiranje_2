use super::models::Sequence;
use crate::structs::range::Range;

/// A sequence that takes two sequences and uses their maximum term by term.
pub struct Max {
    pub sequences: Vec<Box<dyn Sequence<f64>>>,
}

impl Max {
    /// Creates a new `max` sequence.
    pub fn new(sequences: Vec<Box<dyn Sequence<f64>>>) -> Box<Max> {
        Box::new(Max { sequences })
    }
}

impl Sequence<f64> for Max {
    fn range(&self, range: &Range) -> Vec<f64> {
        let size: usize = (range.to - range.from) as usize;
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
                    if result[index] < vector[index] {
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
