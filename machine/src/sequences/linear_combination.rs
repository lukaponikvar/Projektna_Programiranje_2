use super::models::Sequence;
use crate::{functions::size::size, structs::range::Range};

/// Linear combination of two sequences `a` and `b` as given by: `(x*a_i + y*b_i + w)_i` where `x`, `y` and `w` are the three parameters.
pub struct LinearCombination {
    pub parameters: Vec<f64>,
    pub sequences: Vec<Box<dyn Sequence<f64>>>,
}

impl LinearCombination {
    /// Creates a new `linear_combination` sequence.
    pub fn new(
        parameters: Vec<f64>,
        sequences: Vec<Box<dyn Sequence<f64>>>,
    ) -> Box<LinearCombination> {
        Box::new(LinearCombination {
            parameters,
            sequences,
        })
    }
}

impl Sequence<f64> for LinearCombination {
    fn range(&self, range: &Range) -> Vec<f64> {
        let size: usize = size(range);
        if self.parameters.len() != self.sequences.len() + 1 || self.sequences.len() == 0 {
            vec![0.0; size]
        } else {
            let mut vectors = Vec::new();
            for seq in &self.sequences {
                vectors.push(seq.range(&range))
            }
            let mut result = vec![0.0; size];
            for (pos, vector) in vectors.into_iter().enumerate() {
                for index in 0..size {
                    result[index] += self.parameters[pos] * vector[index];
                }
            }
            for index in 0..size {
                result[index] += self.parameters[self.parameters.len() - 1]
            }
            result
        }
    }
}
