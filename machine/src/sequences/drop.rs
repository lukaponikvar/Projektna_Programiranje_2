use super::models::Sequence;
use crate::structs::range::Range;

pub struct Drop {
    pub shift: u64,
    pub sequence: Box<dyn Sequence<f64, dyn Send> + Send>,
}

impl Drop {
    pub fn new(shift: u64, sequence: Box<dyn Sequence<f64, dyn Send> + Send>) -> Box<Drop> {
        Box::new(Drop { shift, sequence })
    }
}

impl Sequence<f64, dyn Send> for Drop {
    fn range(&self, range: &Range) -> Vec<f64> {
        self.sequence.range(&Range {
                    from: range.from + self.shift,
                    to: range.to + self.shift,
                    step: range.step,
                })
    }
}
