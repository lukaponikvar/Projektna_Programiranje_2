use crate::structs::range::Range;

pub trait Sequence<T> {
    fn description(&self) -> String;
    fn k_th(&self, k: u64) -> T;
    fn range(&self, range: Range) -> Vec<T>;
}
