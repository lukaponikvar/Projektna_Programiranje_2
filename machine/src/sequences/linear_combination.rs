use super::models::Sequence;
use crate::structs::range::Range;

// a ne bi tole rabil še 2 parametrov?
// na spletki majo 3 parametre...

/// Linear combination of two sequences `a` and `b` as given by: `(x*a_i + y*b_i)_i` where `x` and `y` are two parameters.
pub struct LinearCombination {
    pub sequences: Vec<Box<dyn Sequence<f64, dyn Send> + Send>>,
}

impl LinearCombination {
    /// Creates a new `linear_combination` sequence.
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
