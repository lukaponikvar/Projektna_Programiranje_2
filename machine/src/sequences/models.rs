use crate::structs::range::Range;
use hyper::Error;

pub trait Sequence<T> {
    async fn range(&self, range: Range) -> Vec<T>;
}
