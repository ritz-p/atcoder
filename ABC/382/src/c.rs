use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize;n],
        b: [usize;m]
    };
    let mut res = vec![-1; m];
    let mut bmap = BTreeMap::new();

    for i in 0..m {
        bmap.entry(b[i]).or_insert(vec![]).push(i);
    }
    for i in 0..n {
        let mut keys = vec![];
        for (&k, v) in bmap.range_mut(a[i]..) {
            for e in v {
                res[*e] = i as isize + 1;
            }
            keys.push(k);
        }
        for k in keys {
            bmap.remove(&k);
        }
    }
    println!("{}", res.iter().join("\n"));
}
