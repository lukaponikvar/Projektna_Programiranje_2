use crate::structs::range::Range;

pub fn size(range: &Range) -> usize {
    if range.to <= range.from {
        0
    } else {
        (((range.to - range.from - 1) / range.step) + 1) as usize
    }
}
