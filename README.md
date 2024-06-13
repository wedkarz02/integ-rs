# integ-rs

This repo is one of the projects for Numerical Algorithms course I'm taking at my University. It implements and compares some algorithms for calculating an integral on a given interval. 
Implemented algorithms:
 - Rectangle rule
 - Trapezoid rule
 - Simpson's rule

# Requirements
 - [Rust](https://www.rust-lang.org/)
 - [Cargo](https://doc.rust-lang.org/cargo/)
 - [Python 3+](https://www.python.org/)
 - [matplotlib](https://matplotlib.org/)
 - Linux OS (preferably)

# Quick Setup

Download this repository using:
```bash
$ git clone https://github.com/wedkarz02/integ-rs.git
```
or use the *Download ZIP* option from the GitHub repository page.

To compile the project, use *cargo build*. I highly recommend compiling in *--release* mode due to better execution speed.

```bash
$ cargo build --release
```
Run the executable directly or with *cargo run --release*. See [src/main.rs](https://github.com/wedkarz02/integ-rs/blob/main/src/main.rs) for avaliable command line arguments.

# License

If not directly stated otherwise, everything in this project is under the MIT License. See the [LICENSE](https://github.com/wedkarz02/integ-rs/blob/main/LICENSE) file for more info.

# Cool links and references
 - Numerical Algorithms lecture presentations
 - https://en.wikipedia.org/wiki/Numerical_integration
 - https://en.wikipedia.org/wiki/Riemann_sum
 - https://en.wikipedia.org/wiki/Trapezoidal_rule
 - https://en.wikipedia.org/wiki/Simpson%27s_rule
 - https://en.wikipedia.org/wiki/Risch_algorithm
