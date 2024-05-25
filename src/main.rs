use csv::{ReaderBuilder, WriterBuilder};
use serde::Deserialize;
use std::env;
use std::error::Error;
use std::f64::consts::PI;
use std::fs::File;
use std::process::{self, Command};

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
    2.0 * integrate::simpson(-radius, radius, div, |x| (1.0 - x.powf(2.0)).sqrt())
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
                .arg("scripts/plot_pi.py")
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
        "parabola" => {
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
        _ => eprintln!("Unrecognised optional argument"),
    };
}
