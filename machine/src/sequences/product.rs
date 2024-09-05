use super::models::Sequence;
use crate::structs::range::Range;

pub struct Product {
    pub sequences: Vec<Box<dyn Sequence<f64, dyn Send> + Send>>,
}

impl Product {
    pub fn new(sequences: Vec<Box<dyn Sequence<f64, dyn Send> + Send>>) -> Box<Product> {
        Box::new(Product { sequences })
    }
}

impl Sequence<f64, dyn Send> for Product {
    fn range(&self, range: &Range) -> Vec<f64> {
        println!("prod{}", self.sequences.len());
        let size: usize = (range.to - range.from + 1) as usize;
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
