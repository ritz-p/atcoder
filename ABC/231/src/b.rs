use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [String;n]
    };

    let mut map = HashMap::new();
    for cs in s {
        *map.entry(cs).or_insert(0) += 1;
    }
    let mut current = 0;
    let mut res = "".to_string();
    for (k, v) in map {
        if v > current {
            current = v;
            res = k;
        }
    }
    println!("{}", res);
}
