use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        a: usize,
        b: usize,
        c: usize,
        d: usize,
    };
    let mut map = HashMap::new();
    *map.entry(a).or_insert(0) += 1;
    *map.entry(b).or_insert(0) += 1;
    *map.entry(c).or_insert(0) += 1;
    *map.entry(d).or_insert(0) += 1;

    let mut x = 0;
    let mut y = 0;
    for (_index, e) in map {
        if x == 0 {
            x = e;
            continue;
        }
        if x > 0 && y == 0 {
            y = e;
        }
    }

    if (x == 2 && y == 2) || (x == 1 && y == 3) || (x == 3 && y == 1) {
        println!("Yes");
    } else {
        println!("No");
    }
}
