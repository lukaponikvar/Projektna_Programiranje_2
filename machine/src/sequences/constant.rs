use crate::sequences::models::Sequence;


pub struct Constant<T> {
    vrednost: T,
}

impl Constant<i64> {
    pub fn new(vrednost: i64) -> Constant<i64> {
        Constant{
            vrednost,
        }
    }
}
impl Sequence<i64> for Constant<i64> {
    fn name(&self) -> String {
        let niz = format!("Konstantno zaporedje {}", self.vrednost);
        niz
    }

    fn start(&self) -> Option<i64> {
        Some(self.vrednost)
    }

    fn k_th(&self, k: usize) -> Option<i64> {
        Some(self.vrednost)
    }

    fn contains(&self, item: i64) -> bool {
        self.vrednost == item
    }
}