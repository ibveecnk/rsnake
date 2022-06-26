pub fn signum(x: f64) -> f64 {
    if x == 0.0 {
        return 0.0;
    } else if x < 0.0 {
        return -1.0;
    } else {
        return 1.0;
    }
}
