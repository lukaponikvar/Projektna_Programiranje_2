use super::models::Sequence;
use crate::structs::range::Range;

pub struct Sum {
    pub sequences: Vec<Box<dyn Sequence<f64, dyn Send> + Send>>,
}

impl Sum {
    pub fn new(sequences: Vec<Box<dyn Sequence<f64, dyn Send> + Send>>) -> Box<Sum> {
        Box::new(Sum { sequences })
    }
}

impl Sequence<f64, dyn Send> for Sum {
    fn range(&self, range: &Range) -> Vec<f64> {
        let size: usize = (range.to - range.from) as usize;
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
