use super::models::Sequence;
use crate::{functions::size::size, structs::range::Range};

/// A sequence that takes two sequences and multiplies them term by term.
pub struct Product {
    pub sequences: Vec<Box<dyn Sequence<f64>>>,
}

impl Product {
    /// Creates a new `product` sequence.
    pub fn new(sequences: Vec<Box<dyn Sequence<f64>>>) -> Box<Product> {
        Box::new(Product { sequences })
    }
}

impl Sequence<f64> for Product {
    fn range(&self, range: &Range) -> Vec<f64> {
        let size: usize = size(range);
        if self.sequences.len() == 0 {
            vec![1.0; size]
        } else {
            let mut vectors = Vec::new();
            for seq in &self.sequences {
                vectors.push(seq.range(&range))
            }
            let mut result = vec![1.0; size];
            for vector in &vectors {
                for index in 0..size {
                    result[index] *= vector[index];
                }
            }
            result
        }
    }
}
