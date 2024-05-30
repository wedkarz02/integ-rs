use csv::{ReaderBuilder, WriterBuilder};
use serde::Deserialize;
use std::env;
use std::error::Error;
use std::f64::consts::{E, PI};
use std::fs::File;
use std::process::{self, Command};

pub mod compare;
pub mod integrate;

#[derive(Debug, Deserialize)]
struct Record {
    n: u32,
    pi: f64,
    verr: f64,
}

fn update_pi_file(path: &str, data: &[Vec<f64>]) -> Result<(), Box<dyn Error>> {
    let file = File::open(path)?;
    let mut reader = ReaderBuilder::new().delimiter(b';').from_reader(file);

    let mut records: Vec<Record> = Vec::new();
    let mut row_count = 0;

    for result in reader.deserialize() {
        let record: Record = result?;
        records.push(record);
        row_count += 1;
    }

    if data.len() != row_count {
        return Err(format!(
            "Data length != number of csv rows \
            (left: {}, right: {})",
            data.len(),
            row_count - 1
        )
        .into());
    }

    let out_file = File::create(path)?;
    let mut writer = WriterBuilder::new().delimiter(b';').from_writer(out_file);

    writer.write_record(["n", "pi", "verr", "int", "ierr"])?;

    for (i, record) in records.into_iter().enumerate() {
        let mut record_vec = vec![
            record.n.to_string(),
            record.pi.to_string(),
            record.verr.to_string(),
        ];

        record_vec.extend(data[i].iter().map(|&x| x.to_string()));
        writer.write_record(&record_vec)?;
    }

    writer.flush()?;
    Ok(())
}

#[inline]
fn circle_area(radius: f64, div: u32) -> f64 {
    2.0 * integrate::simpson(-radius, radius, div, |x| (1.0 - x.powi(2)).sqrt())
}

#[inline]
fn ellipse_area(a: f64, b: f64, div: u32) -> f64 {
    2.0 * integrate::simpson(-a, a, div, |x| b * (1.0 - x.powi(2) / a.powi(2)).sqrt())
}

#[inline]
fn arc_length(a: f64, b: f64, div: u32, eps: f64, derivative: impl Fn(&f64) -> f64) -> f64 {
    integrate::rectangle(a + eps, b - eps, div, |x| {
        (1.0 + derivative(x).powi(2)).sqrt()
    })
}

#[inline]
fn ellipse_arc(a: f64, b: f64, div: u32, eps: f64) -> f64 {
    2.0 * arc_length(-a, a, div, eps, |x| {
        (-b * x) / (a.powi(2) * (1.0 - x.powi(2) / a.powi(2)).sqrt())
    })
}

fn calculate_iter_pi() -> Vec<Vec<f64>> {
    let mut out = vec![];

    for i in 2..=25 {
        let n = 2u32.pow(i);
        let pi = circle_area(1.0, n);
        let diff = (PI - pi).abs();
        out.push(vec![pi, diff]);
    }

    out
}

fn display_ellipse_area(a: f64, b: f64, div: u32, actual: f64) {
    let area = ellipse_area(a, b, div);
    println!("Ellipse area: {}", area);
    println!(
        "a: {}, b: {}, div: {}\nactual: {}, diff: {:e}\n",
        a,
        b,
        div,
        actual,
        (actual - area).abs()
    );
}

fn run_parabola() {
    let div = 100_000;
    let actual = 1.0 / 3.0;
    let area = integrate::simpson(0.0, 1.0, div, |x| x.powf(2.0));
    println!(
        "Calculated area under a parabola on an interval [0; 1] = {}",
        area
    );
    println!(
        "With {} divisions, actual value: {}, difference: {:e}",
        div,
        actual,
        (actual - area).abs()
    );
}

fn run_ellipse() {
    let div = 100_000;
    display_ellipse_area(1.0, 1.0, div, PI);
    display_ellipse_area(3.0, 2.0, div, 6.0 * PI);
    display_ellipse_area(7.0, 3.0, div, 21.0 * PI);
    display_ellipse_area(4.0, 1.0, div, 4.0 * PI);
}

fn run_sin() {
    let div = 100_000;
    let area = integrate::simpson(0.0, 1.0, div, |x| x.sin());
    let actual = 1.0 - 1f64.cos();
    println!(
        "Calculated area under a sine on an interval [0; 1] = {}",
        area
    );
    println!(
        "With {} divisions, actual value: {}, difference: {:e}",
        div,
        actual,
        (actual - area).abs()
    );
}

fn run_arc_pi() {
    let div = 1_000_000;
    let eps = 1e-16;
    let circ = ellipse_arc(1.0, 1.0, div, eps);
    println!("Circumference of a circle with a radius of 1 = {}", circ);
    println!(
        "With {} divisions, actual value: {}, difference: {:e}",
        div,
        PI * 2.0,
        (PI * 2.0 - circ).abs()
    );
    println!(
        "Approximated pi: {}, difference: {:e}",
        circ / 2.0,
        (PI - circ / 2.0).abs()
    );
}

fn display_ellipse_perimeter(a: f64, b: f64, div: u32, eps: f64, actual: f64) {
    let arc = ellipse_arc(a, b, div, eps);
    println!("Ellipse perimeter: {}", arc);
    println!(
        "a: {}, b: {}, div: {}\nactual: {}, diff: {:e}\n",
        a,
        b,
        div,
        actual,
        (actual - arc).abs()
    );
}

fn run_arc_ellipse() {
    let div = 1_000_000;
    let eps = 1e-10;
    display_ellipse_perimeter(3.0, 2.0, div, eps, 15.8654);
    display_ellipse_perimeter(7.0, 3.0, div, eps, 32.6857);
    display_ellipse_perimeter(4.0, 1.0, div, eps, 17.1568);
}

fn run_sin_arc() {
    let div = 1_000_000;
    let eps = 1e-16;
    let arc = arc_length(0.0, 2.0 * PI, div, eps, |x| x.cos());
    let actual = 7.640395578055424;
    println!("Length of the sine curve on an interval [0; 2pi] = {}", arc);
    println!(
        "with {} divisions, actual value: {}, difference: {:e}",
        div,
        actual,
        (actual - arc).abs()
    );
}

fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Nothing to do...");
        process::exit(0);
    }

    match args[1].as_str() {
        "pi" => {
            let pi_vec = calculate_iter_pi();
            if let Err(e) = update_pi_file("dump/pi_res.csv", &pi_vec) {
                eprintln!("{}", e);
                process::exit(0);
            }

            let py_output = Command::new("python3")
                .arg("scripts/plotter.py")
                .arg("pi")
                .output()
                .expect("failed to execute python process");

            if py_output.status.success() {
                let result = String::from_utf8_lossy(&py_output.stdout);
                println!("{}", result);
            } else {
                let result = String::from_utf8_lossy(&py_output.stderr);
                eprintln!("{}", result);
            }
        }
        "areas" => {
            println!("\nPARABOLA:");
            run_parabola();
            println!("\n\nELLIPSE:");
            run_ellipse();
            println!("\nSINE:");
            run_sin();
        }
        "arcs" => {
            println!("\nCIRCUMFERENCE & PI:");
            run_arc_pi();
            println!("\n\nELLIPSE PERIMETER:");
            run_arc_ellipse();
            println!("\nSINE:");
            run_sin_arc();
        }
        "compare" => {
            compare::inc_dump("dump/x2.csv", -1.0, 1.0, 2.0 / 3.0, |x| x.powi(2)).unwrap();
            compare::inc_dump("dump/sin.csv", 0.0, PI, 2.0, |x| x.sin()).unwrap();
            compare::inc_dump("dump/ex.csv", 0.0, 1.0, E - 1.0, |x| E.powf(*x)).unwrap();
            compare::inc_dump("dump/1x.csv", 1.0, 2.0, 2f64.ln(), |x| 1.0 / x).unwrap();
            compare::inc_dump("dump/cos.csv", 0.0, PI / 2.0, 1.0, |x| x.cos()).unwrap();

            let py_output = Command::new("python3")
                .arg("scripts/plotter.py")
                .arg("all")
                .output()
                .expect("failed to execute python process");

            if py_output.status.success() {
                let result = String::from_utf8_lossy(&py_output.stdout);
                println!("{}", result);
            } else {
                let result = String::from_utf8_lossy(&py_output.stderr);
                eprintln!("{}", result);
            }
        }
        _ => eprintln!("Unrecognised optional argument"),
    };
}
