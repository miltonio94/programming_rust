use num::Complex;
use std::str::FromStr;

fn main() {
    println!("Hello, world!");
}

fn comples_square_add_loop(c: Complex<u64>) {
    let mut z: Complex<f64> = Complex { re: 0.0, im: 0.0 };
    loop {
        z = z * z + c;
    }
}

fn escape_time(c: Complex<u64>, limit: usize) -> Option<usize> {
    let mut z: Complex<u64> = Complex { re: 0.0, im: 0.0 };

    for i in 0..limit {
        if z.norm_sqr() > 4.0 {
            return Some(i);
        }
        z = z * z + c;
    }
    None
}

fn parse_pair<T: FromStr>(s: &str, separator: char) -> Option<(T, T)> {
    //
    match s.find(seperator) {
        None => None,
        Some(i) => None,
    }
}
