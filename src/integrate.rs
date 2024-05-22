pub fn run_fn(msg: &str, f: impl Fn(&str)) {
    f(msg);
}

pub fn add_vals(a: f32, b: f32, f: impl Fn(&f32) -> f32) -> f32 {
    f(&a) + f(&b)
}
