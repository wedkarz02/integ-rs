/// # Panics
/// This function will panic if `a` is greater than `b` or if `n` is 0.
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
        let eps = 1e-4;
        let div = 100;
        assert_loosely_eq!(
            2.0 / 3.0,
            rectangle_rule(-1.0, 1.0, div, |x| x.powf(2.0)),
            eps
        );
        assert_loosely_eq!(2.0, rectangle_rule(0.0, PI, div, |x| x.sin()), eps);
        assert_loosely_eq!(E - 1.0, rectangle_rule(0.0, 1.0, div, |x| E.powf(*x)), eps);
        assert_loosely_eq!(2f64.ln(), rectangle_rule(1.0, 2.0, div, |x| 1.0 / x), eps);
        assert_loosely_eq!(1.0, rectangle_rule(0.0, PI / 2.0, div, |x| x.cos()), eps);
    }
}
