use super::models::Sequence;
use crate::structs::range::Range;

pub struct Arithmetic {
    pub start: f64,
    pub step: f64,
}

impl Arithmetic {
    pub fn new(start: f64, step: f64) -> Box<Arithmetic> {
        Box::new(Arithmetic { start, step })
    }

    pub fn k_th(&self, k: u64) -> f64 {
        self.start + (k as f64) * self.step
    }
}

impl Sequence<f64, dyn Send> for Arithmetic {
    fn range(&self, range: &Range) -> Vec<f64> {
        let mut result = Vec::new();
        let mut index = range.from;
        while index < range.to {
            result.push(self.k_th(index as u64));
            index += range.step;
        }
        result
    }
}
