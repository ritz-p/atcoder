use std::collections::BTreeMap;

use proconio::input;
fn main() {
    input! {
        n: usize,
        sc: [(usize,usize);n]
    };
    let mut bmap = BTreeMap::new();

    let mut res = 0;

    for (s, c) in sc {
        bmap.insert(s, c);
    }

    while let Some((s, c)) = bmap.pop_first() {
        res += c % 2;
        if c > 1 {
            *bmap.entry(s * 2).or_insert(0) += c / 2;
        }
    }

    println!("{}", res);
}
