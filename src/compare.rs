use std::{fs, io};

use crate::integrate::*;

fn compare_on_fn(a: f64, b: f64, div: u32, actual: f64, f: impl Fn(&f64) -> f64) -> Vec<f64> {
    let mut errors: Vec<f64> = Vec::new();

    let rec_res = rectangle(a, b, div, &f);
    let rec_err = (actual - rec_res).abs();
    let trp_res = trapezoid(a, b, div, &f);
    let trp_err = (actual - trp_res).abs();
    let sim_res = simpson(a, b, div, &f);
    let sim_err = (actual - sim_res).abs();

    errors.push(rec_err);
    errors.push(trp_err);
    errors.push(sim_err);

    errors
}

pub fn inc_dump(
    path: &str,
    a: f64,
    b: f64,
    actual: f64,
    f: impl Fn(&f64) -> f64,
) -> io::Result<()> {
    let mut rows: Vec<String> = Vec::new();
    rows.push(String::from("n;rec;trp;sim"));

    for i in 1..=27 {
        let div = 2u32.pow(i);
        let errors = compare_on_fn(a, b, div, actual, &f);
        let str_row = format!("{};{};{};{}", div, errors[0], errors[1], errors[2]);
        rows.push(str_row);
    }

    let contents = rows.join("\n");
    fs::write(path, contents)?;

    Ok(())
}
