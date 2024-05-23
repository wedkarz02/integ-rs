/// # Panics
/// This function will panic if `a` is greater than `b` or if `n` is 0.
pub fn rectangle(a: f64, b: f64, n: u32, f: impl Fn(&f64) -> f64) -> f64 {
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

/// # Panics
/// This function will panic if `a` is greater than `b` or if `n` is 0.
pub fn trapezoid(a: f64, b: f64, n: u32, f: impl Fn(&f64) -> f64) -> f64 {
    assert!(a <= b);
    assert!(n > 0);

    let height = (b - a) / n as f64;

    let mut sum = 0f64;
    sum += f(&a) / 2.0;
    for i in 1..n {
        sum += f(&(a + i as f64 * height));
    }
    sum += f(&b) / 2.0;

    sum * height
}

/// # Panics
/// This function will panic if `a` is greater than `b` or if `n` is 0 or if `n` is odd.
pub fn simpson(a: f64, b: f64, n: u32, f: impl Fn(&f64) -> f64) -> f64 {
    assert!(a <= b);
    assert!(n > 0);
    assert!(n & 1 == 0);

    let height = (b - a) / n as f64;

    let x: Vec<f64> = (0..=n).into_iter().map(|i| a + i as f64 * height).collect();
    let y: Vec<f64> = x.iter().map(|xi| f(xi)).collect();
    let sum: f64 = y[0]
        + y[y.len() - 1]
        + 4.0 * (1..n).step_by(2).map(|i| y[i as usize]).sum::<f64>()
        + 2.0 * (2..n - 1).step_by(2).map(|i| y[i as usize]).sum::<f64>();

    sum * (height / 3.0)
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f64::consts::{E, PI};

    macro_rules! assert_loosely_eq {
        ($a:expr, $b:expr, $eps:expr) => {
            if ($a - $b).abs() > $eps {
                panic!(
                    "assertion failed: `(left ~= right)` \
                (left: `{}`, right: `{}`, eps: `{}`)",
                    $a, $b, $eps
                );
            }
        };
    }

    #[test]
    fn test_rectangle() {
        let eps = 1e-8;
        let div = 10_000;
        assert_loosely_eq!(2.0 / 3.0, rectangle(-1.0, 1.0, div, |x| x.powf(2.0)), eps);
        assert_loosely_eq!(2.0, rectangle(0.0, PI, div, |x| x.sin()), eps);
        assert_loosely_eq!(E - 1.0, rectangle(0.0, 1.0, div, |x| E.powf(*x)), eps);
        assert_loosely_eq!(2f64.ln(), rectangle(1.0, 2.0, div, |x| 1.0 / x), eps);
        assert_loosely_eq!(1.0, rectangle(0.0, PI / 2.0, div, |x| x.cos()), eps);
    }

    #[test]
    fn test_trapezoid() {
        let eps = 1e-7;
        let div = 10_000;
        assert_loosely_eq!(2.0 / 3.0, trapezoid(-1.0, 1.0, div, |x| x.powf(2.0)), eps);
        assert_loosely_eq!(2.0, trapezoid(0.0, PI, div, |x| x.sin()), eps);
        assert_loosely_eq!(E - 1.0, trapezoid(0.0, 1.0, div, |x| E.powf(*x)), eps);
        assert_loosely_eq!(2f64.ln(), trapezoid(1.0, 2.0, div, |x| 1.0 / x), eps);
        assert_loosely_eq!(1.0, trapezoid(0.0, PI / 2.0, div, |x| x.cos()), eps);
    }

    #[test]
    fn test_simpson() {
        let eps = 1e-14;
        let div = 10_000;
        assert_loosely_eq!(2.0 / 3.0, simpson(-1.0, 1.0, div, |x| x.powf(2.0)), eps);
        assert_loosely_eq!(2.0, simpson(0.0, PI, div, |x| x.sin()), eps);
        assert_loosely_eq!(E - 1.0, simpson(0.0, 1.0, div, |x| E.powf(*x)), eps);
        assert_loosely_eq!(2f64.ln(), simpson(1.0, 2.0, div, |x| 1.0 / x), eps);
        assert_loosely_eq!(1.0, simpson(0.0, PI / 2.0, div, |x| x.cos()), eps);
    }
}
