use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [[usize;n];n]
    };
    let mut v = vec![BTreeSet::new(); n];
    for i in 0..n {
        for j in 0..n {
            if a[i][j] == 1 {
                v[i].insert(j + 1);
                v[j].insert(i + 1);
            }
        }
    }
    for i in 0..n {
        println!("{}", v[i].iter().join(" "));
    }
}
