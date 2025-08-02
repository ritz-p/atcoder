use std::collections::BTreeMap;

use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize;n],
        b: [usize;m]
    };
    let mut bmap = BTreeMap::new();

    for e in a {
        *bmap.entry(e).or_insert(0) += 1;
    }
    for e in b {
        if let Some(v) = bmap.get_mut(&e) {
            *v -= 1;
            if *v == 0 {
                bmap.remove(&e);
            }
        }
    }
    while let Some((k, v)) = bmap.pop_first() {
        for _i in 0..v {
            print!("{} ", k);
        }
    }
    println!();
}
