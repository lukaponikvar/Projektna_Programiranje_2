use super::models::Sequence;
use crate::structs::range::Range;

/// A sequence that takes a sequences and calculates terms by taking terms, rounding them and converting the integers to binary.
pub struct Binary {
    pub sequence: Box<dyn Sequence<f64>>,
}

impl Binary {
    /// Creates a new `drop` sequence.
    pub fn new(sequence: Box<dyn Sequence<f64>>) -> Box<Binary> {
        Box::new(Binary { sequence })
    }
}

impl Sequence<f64> for Binary {
    fn range(&self, range: &Range) -> Vec<f64> {
        let vector = self.sequence.range(range);
        vector
            .iter()
            .map(|x| {
                format!("{:b}", *x as i64)
                    .parse()
                    .expect("This can't happen!")
            })
            .collect()
    }
}
