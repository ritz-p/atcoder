use std::collections::BTreeMap;

use proconio::input;
fn main() {
    input! {
        n: usize,
        sc: [(usize,usize);n]
    };
    let mut bmap = BTreeMap::new();

    for (s, c) in sc {
        *bmap.entry(s).or_insert(0) += c;
    }
    let mut res = 0;

    while let Some((k, v)) = bmap.pop_first() {
        if v > 1 {
            *bmap.entry(k * 2).or_insert(0) += v / 2;
        }
        res += v % 2;
    }

    println!("{}", res);
}
