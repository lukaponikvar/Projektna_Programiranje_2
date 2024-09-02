fn power_plus(x: i64, y: i64) -> i64 {
    if y == 0 {1} else {x * power(x, y-1)}
}

pub fn power(x: i64, y: i64) -> i64 {
    if y >= 0 {power_plus(x, y)} else {
        1/ power_plus(x, -y)
    }
}