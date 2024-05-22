pub mod integrate;

fn display(text: &str) {
    println!("{}", text);
}

fn f(x: &f32) -> f32 {
    x.powf(2f32) + 1f32
}

fn main() {
    integrate::run_fn("hello", display);
    let res = integrate::add_vals(1f32, 2f32, f);
    integrate::run_fn(&res.to_string(), display);
}
