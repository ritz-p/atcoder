use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        t: usize,
    };

    let mut res = vec![];

    for _ in 0..t {
        input! {
            n: usize,
            tc: [usize;n]
        };
        let mut bset = BTreeSet::new();
        for t in tc.iter().skip(1).take(n - 2) {
            bset.insert(*t);
        }
        let mut current = tc[0];
        let mut count = 2;
        while current * 2 < tc[n - 1] {
            if let Some(v) = bset.range(..=current * 2).next_back() {
                if *v == current {
                    count = -1;
                    break;
                }
                current = *v;
                count += 1;
            } else {
                count = -1;
                break;
            }
        }
        res.push(count);
    }

    println!("{}", res.iter().join("\n"));
}
