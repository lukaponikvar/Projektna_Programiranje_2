use crate::expressions::models::AExpr;
use crate::sequences::models::Sequence;
use std::collections::HashMap;

pub struct Combined<'a, T> {
    sequences: Vec<Box<&'a dyn Sequence<T>>>,
    expression: AExpr,
}

impl Sequence<i64> for Combined<'_, i64> {
    fn name(&self) -> String {
        let mut niz = format!("Combined");
        for zaporedje in self.sequences.iter() {
            niz = format!("{} {},", niz, (*zaporedje).name())
        };
        niz
    }

    fn start(&self) -> Option<i64> {
        let mut hash = HashMap::new();
        for zaporedje in self.sequences.iter() {
            hash.insert(zaporedje.name(), zaporedje.start());
        };
        self.expression.evaluate_map(&hash)
    }

    fn k_th(&self, k: usize) -> Option<i64> {
        let mut hash = HashMap::new();
        for zaporedje in self.sequences.iter() {
            hash.insert(zaporedje.name(), zaporedje.k_th(k));
        };
        self.expression.evaluate_map(&hash)
    }

    fn contains(&self, item: i64) -> bool {
        panic!("Shifted")
    }
}

impl<T> Combined<'_, T> {
    fn new(sequences: Vec<Box<&dyn Sequence<T>>>, expression: AExpr) -> Box<Combined<T>> {
        Box::new(
            Combined{
                sequences,
                expression,
            }
        )
    }
}

pub fn combined_sequence(
    sequences: Vec<Box<&dyn Sequence<i64>>>,
    expression: AExpr,
) -> Box<dyn Sequence<i64> + '_> {
    Combined::new(sequences, expression)
}
