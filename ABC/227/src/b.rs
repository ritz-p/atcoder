use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize;n]
    };
    let m = 1000 / 7 + 1;
    let mut set = HashSet::new();
    for a in 1..=m {
        for b in 1..=m {
            let c = 4 * a * b + 3 * a + 3 * b;
            if c <= 1000 {
                set.insert(c);
            }
        }
    }
    let mut res = 0;

    for c in s {
        if !set.contains(&c) {
            res += 1;
        }
    }

    println!("{}", res);
}
