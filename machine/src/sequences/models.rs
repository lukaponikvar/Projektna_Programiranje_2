use crate::structs::range::Range;

pub trait Sequence<T> {
    fn range(&self, range: Range) -> Vec<T>;
}
