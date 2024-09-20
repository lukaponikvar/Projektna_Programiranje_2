/// Returns a `base` raised to a given `power`.
pub fn power(base: f64, exponent: u64) -> f64 {
    if exponent == 0 {
        1.0
    } else if exponent == 1 {
        base
    } else if exponent == 2 {
        base * base
    } else if exponent % 2 == 0 {
        power(base * power(base, exponent / 2), 2)
    } else {
        power(base * power(base, exponent / 2), 2) * base
    }
}
