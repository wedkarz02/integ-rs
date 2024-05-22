use std::f64::consts::{E, PI};

pub mod integrate;

fn f(x: &f64) -> f64 {
    x.powf(2f64)
}

fn main() {
    let iterations = 100;

    let res = integrate::rectangle_rule(-1.0, 1.0, iterations, f);
    println!("{}", res);

    let res = integrate::rectangle_rule(-1.0, 1.0, iterations, |x| x.powf(2f64));
    println!("{}", res);

    let res = integrate::rectangle_rule(0.0, PI, iterations, |x| x.sin());
    println!("{}", res);

    let res = integrate::rectangle_rule(0.0, 1.0, iterations, |x| E.powf(*x));
    println!("{}", res);

    let res = integrate::rectangle_rule(1.0, 2.0, iterations, |x| 1.0 / x);
    println!("{}", res);

    let res = integrate::rectangle_rule(0.0, PI / 2.0, iterations, |x| x.cos());
    println!("{}", res);
}
