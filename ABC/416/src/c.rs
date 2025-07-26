use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        n: usize,
        k: usize,
        x: usize,
        mut s: [String;n]
    };
    s.sort();
    let mut bmap = BTreeMap::new();
    for comb in (0..k).map(|_| 0..n).multi_cartesian_product() {
        let mut ss = "".to_string();
        for c in comb {
            ss += &s[c];
        }
        *bmap.entry(ss).or_insert(0) += 1;
    }
    let mut current = 0;

    while let Some((k, v)) = bmap.pop_first() {
        if current < x {
            current += v;
        }
        if current >= x {
            println!("{}", k);

            return;
        }
    }
}
