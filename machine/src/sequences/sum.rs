use super::models::Sequence;
use crate::structs::range::Range;
// use std::error::Error;

pub struct Sum {
    pub sequences: Vec<Box<dyn Sequence<f64, dyn Send> + Send>>,
}

// Vec<Box<&'a dyn Sequence<f64>>>

impl Sum {
    pub fn new(sequences: Vec<Box<dyn Sequence<f64, dyn Send> + Send>>) -> Box<Sum> {
        Box::new(Sum { sequences })
    }
}

impl Sequence<f64, dyn Send> for Sum {
    fn range(&self, _range: Range) -> Vec<f64> {
        return vec![1.0];
    }
}
