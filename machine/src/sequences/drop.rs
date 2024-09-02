use super::models::Sequence;

struct Shifted<'a, T> {
    shift: usize,
    base_sequence: &'a dyn Sequence<T>
}

impl Sequence<i64> for Shifted<'_, i64> {
    fn name(&self) -> String {
        format!("Shifted {} for {} ", self.base_sequence.name(), self.shift)
    }

    fn start(&self) -> Option<i64> {
        if self.shift >= 0 {
            self.base_sequence.k_th(self.shift)
        } else {None}
    }

    fn k_th(&self, k: usize) -> Option<i64> {
        if k + self.shift >= 0 {
            self.base_sequence.k_th(k+self.shift)
        } else {None}
    }

    fn contains(&self, item: i64) -> bool {
        for indeks in 0..10000001 {
            if Some(item) == self.k_th(indeks) {
                return true;
            }
        };
        false
    }
}

impl<T> Shifted<'_, T> {
    fn new(shift: usize, base_sequence: &dyn Sequence<T>) -> Box<Shifted<T>> {
        panic!("Shifted")
    }
}

pub fn shifted_sequence(
    base_sequence: &dyn Sequence<i64>,
    shift: usize,
) -> Box<dyn Sequence<i64> + '_> {
    Shifted::new(shift, base_sequence)
}
