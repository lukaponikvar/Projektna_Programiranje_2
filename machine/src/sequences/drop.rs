use super::models::Sequence;
use crate::structs::range::Range;

/// A sequence which takes a sequence and a shift parameter and is equivalent to shifting that particular sequence by the given amount.
pub struct Drop {
    pub shift: u64,
    pub sequence: Box<dyn Sequence<f64>>,
}

impl Drop {
    /// Creates a new `drop` sequence.
    pub fn new(shift: u64, sequence: Box<dyn Sequence<f64>>) -> Box<Drop> {
        Box::new(Drop { shift, sequence })
    }
}

impl Sequence<f64> for Drop {
    fn range(&self, range: &Range) -> Vec<f64> {
        self.sequence.range(&Range {
            from: range.from + self.shift,
            to: range.to + self.shift,
            step: range.step,
        })
    }
}
