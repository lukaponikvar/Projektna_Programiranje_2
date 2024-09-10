use super::models::Sequence;
use crate::structs::range::Range;

pub struct LinearCombination {
    pub sequences: Vec<Box<dyn Sequence<f64, dyn Send> + Send>>,
}

impl LinearCombination {
    pub fn new(sequences: Vec<Box<dyn Sequence<f64, dyn Send> + Send>>) -> Box<LinearCombination> {
        Box::new(LinearCombination { sequences })
    }
}

impl Sequence<f64, dyn Send> for LinearCombination {
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
