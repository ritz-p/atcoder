use std::collections::HashMap;

use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        mut ab: [(usize,usize);m]
    };
    let mut map = HashMap::new();
    for (a, b) in &ab {
        let k = (a + b) % n;
        *map.entry(k).or_insert(0) += 1;
    }

    let mut res = (1..m).sum::<usize>();

    for (_k, v) in map {
        res -= (v - 1) * v / 2;
    }
    println!("{}", res);
}
