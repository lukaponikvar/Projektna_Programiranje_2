use super::models::Sequence;
use crate::mathematical_functions::power::power;
use crate::structs::range::Range;
use hyper::Error;

pub struct Geometric {
    pub start: f64,
    pub quotient: f64,
}

impl Geometric {
    pub fn new(start: f64, quotient: f64) -> Box<Geometric> {
        Box::new(Geometric { start, quotient })
    }

    pub fn k_th(&self, k: u64) -> f64 {
        self.start * power(self.quotient, k)
    }
}

impl Sequence<f64> for Geometric {
    async fn range(&self, range: Range) -> Vec<f64> {
        let mut result = Vec::new();
        let mut index = range.from;
        while index <= range.to {
            result.push(self.k_th(index as u64));
            index += range.step;
        }
        result
    }
}
