use std::f64::consts::{E, PI};

pub mod integrate;

fn f(x: &f64) -> f64 {
    x.powf(2f64)
}

fn main() {
    let iterations = 100;

    let res = integrate::rectangle_rule(f, -1.0, 1.0, iterations);
    println!("{}", res);

    let res = integrate::rectangle_rule(|x| x.powf(2f64), -1.0, 1.0, iterations);
    println!("{}", res);

    let res = integrate::rectangle_rule(|x| x.sin(), 0.0, PI, iterations);
    println!("{}", res);

    let res = integrate::rectangle_rule(|x| E.powf(*x), 0.0, 1.0, iterations);
    println!("{}", res);

    let res = integrate::rectangle_rule(|x| 1.0 / x, 1.0, 2.0, iterations);
    println!("{}", res);

    let res = integrate::rectangle_rule(|x| x.cos(), 0.0, PI / 2.0, iterations);
    println!("{}", res);
}
