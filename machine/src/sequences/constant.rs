use super::models::Sequence;
use crate::structs::range::Range;

pub struct Constant {
    pub value: f64,
}

impl Constant {
    pub fn new(value: f64) -> Box<Constant> {
        Box::new(Constant { value })
    }
}

impl Sequence<f64, dyn Send> for Constant {
    fn range(&self, range: &Range) -> Vec<f64> {
        let result = vec![self.value; (range.to - range.from + 1) as usize];
        result
    }
}
