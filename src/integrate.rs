/// # Panics
/// This function will panic if a is greater than b or if n is 0.
pub fn rectangle_rule(a: f64, b: f64, n: u32, f: impl Fn(&f64) -> f64) -> f64 {
    assert!(a <= b);
    assert!(n > 0);

    let height = (b - a) / n as f64;
    let mut midpoint = a + (height / 2f64);

    let mut sum = 0f64;
    for _ in 0..n {
        sum += f(&midpoint);
        midpoint += height;
    }

    sum * height
}
