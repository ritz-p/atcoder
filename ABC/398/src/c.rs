use std::collections::BTreeMap;

use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize;n]
    };
    let mut bmap = BTreeMap::new();
    for e in &a {
        *bmap.entry(e).or_insert(0) += 1;
    }
    let mut m = 0;
    while let Some((k, v)) = bmap.pop_last() {
        if v == 1 {
            m = m.max(*k);
        }
    }
    if m == 0 {
        println!("-1");
        return;
    }

    for (index, e) in a.iter().enumerate() {
        if *e == m {
            println!("{}", index + 1);
            return;
        }
    }
}
