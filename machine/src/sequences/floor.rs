use super::models::Sequence;
use crate::structs::range::Range;

/// A sequence that takes one sequences and applies the floor function term by term.
pub struct Floor {
    pub sequence: Box<dyn Sequence<f64>>,
}

impl Floor {
    /// Creates a new `max` sequence.
    pub fn new(sequence: Box<dyn Sequence<f64>>) -> Box<Floor> {
        Box::new(Floor { sequence })
    }
}

impl Sequence<f64> for Floor {
    fn range(&self, range: &Range) -> Vec<f64> {
        let vector: Vec<f64> = (*self.sequence).range(range);
        return vector.iter().map(|x| x.floor()).collect();
    }
}
