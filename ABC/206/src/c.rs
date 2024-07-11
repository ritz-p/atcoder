use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {
        mut n: usize,
        a: [usize;n]
    };

    let mut map = HashMap::new();
    for e in a {
        *map.entry(e).or_insert(0) += 1;
    }
    let mut res = n * (n - 1) / 2;
    for (_k, v) in map {
        res -= v * (v - 1) / 2;
    }
    println!("{}", res);
}
