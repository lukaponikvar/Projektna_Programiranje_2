use crate::structs::range::Range;

pub trait Sequence<T, Send: ?Sized> {
    fn range(&self, range: &Range) -> Vec<T>;
}
