use std::collections::HashMap;

use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        k: usize,
        ss: [Chars;n]
    };
    let mut res = 0;
    for i in 1..=n {
        for comb in ss.iter().combinations(i) {
            let mut map = HashMap::new();
            let mut count = 0;
            for s in comb {
                for c in s {
                    *map.entry(c).or_insert(0) += 1;
                }
            }
            for (_key, value) in map {
                if value == k {
                    count += 1;
                }
            }
            res = res.max(count);
        }
    }

    println!("{}", res);
}
