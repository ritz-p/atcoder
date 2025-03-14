use std::collections::BTreeMap;

use proconio::input;
fn main() {
    input! {
        n: usize,
        mut k: usize,
        ab: [(usize,usize);n]
    };
    let mut bmap = BTreeMap::new();

    for (a, b) in &ab {
        *bmap.entry(a).or_insert(0) += b;
    }

    for (a, b) in bmap.iter() {
        if k > *b {
            k -= b;
        } else {
            println!("{}", a);
            return;
        }
    }
}
