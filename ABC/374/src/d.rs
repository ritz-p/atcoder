use core::f64;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: f64,
        t: f64,
        abcd: [(f64,f64,f64,f64);n]
    };
    let mut res = f64::INFINITY;

    for perm in (0..n).permutations(n) {
        for start in 0..(1 << n) {
            let mut cx = 0.0;
            let mut cy = 0.0;
            let mut total = 0.0;

            for &i in &perm {
                let (a, b, c, d) = if (start >> i) & 1 == 0 {
                    abcd[i]
                } else {
                    let (a, b, c, d) = abcd[i];
                    (c, d, a, b)
                };
                total += distance(cx, cy, a, b) / s;
                total += distance(a, b, c, d) / t;
                cx = c;
                cy = d;
            }
            res = res.min(total);
        }
    }

    println!("{}", res);
}

fn distance(a: f64, b: f64, c: f64, d: f64) -> f64 {
    let x = a - c;
    let y = b - d;
    (x * x + y * y).sqrt()
}
