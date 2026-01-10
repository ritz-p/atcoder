use std::collections::BTreeMap;

use proconio::input;
fn main() {
    input! {
        n: usize,
        t: [usize;n]
    };
    let mut bmap = BTreeMap::new();
    for (i, e) in t.iter().enumerate() {
        bmap.insert(e, i);
    }

    for (i, (_k, v)) in bmap.iter().enumerate() {
        if i == 3 {
            break;
        }
        print!("{} ", v + 1);
    }
    println!();
}
