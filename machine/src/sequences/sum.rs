use super::models::Sequence;
use crate::structs::range::Range;

/// A sequence that takes two sequences and adds them term by term.
pub struct Sum {
    pub sequences: Vec<Box<dyn Sequence<f64, dyn Send> + Send>>,
}

impl Sum {
    /// Creates a new `sum` sequence.
    pub fn new(sequences: Vec<Box<dyn Sequence<f64, dyn Send> + Send>>) -> Box<Sum> {
        Box::new(Sum { sequences })
    }
}

impl Sequence<f64, dyn Send> for Sum {
    fn range(&self, range: &Range) -> Vec<f64> {
        let size: usize = (range.to - range.from) as usize;
        if self.sequences.len() == 0 {
            vec![0.0; size]
        } else {
            let mut vectors = Vec::new();
            for seq in &self.sequences {
                vectors.push(seq.range(&range))
            }
            let mut result = vec![0.0; size];
            for vector in &vectors {
                for index in 0..size {
                    result[index] += vector[index];
                }
            }
            result
        }
    }
}
