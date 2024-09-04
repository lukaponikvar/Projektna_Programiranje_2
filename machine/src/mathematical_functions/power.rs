///Funkcija vrne potenco dane baze na dani eksponent.
pub fn power(base: f64, exponent: u64) -> f64 {
    if exponent == 0 {
        1.0
    } else {
        base * power(base, exponent - 1)
    }
}

// TODO: hitro potenciranje
