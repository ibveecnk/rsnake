// Currently no longer used

pub fn signum(x: f64) -> f64 {
    if x == 0.0 {
        0.0
    } else if x < 0.0 {
        -1.0
    } else {
        1.0
    }
}

pub fn abs(x: f64) -> f64 {
    if x >= 0.0 {
        x
    } else {
        -1.0 * x
    }
}
