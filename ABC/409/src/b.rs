use std::collections::BTreeMap;

use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize;n]
    };
    let mut bmap = BTreeMap::new();

    for i in 0..n {
        *bmap.entry(a[i]).or_insert(0) += 1;
    }
    let mut res = 0;

    for i in 0..n + 1 {
        let mut iter = bmap.range(i..);
        let mut current = 0;

        while let Some((_k, v)) = iter.next() {
            current += v;
            if current >= i {
                res = res.max(i);
                break;
            }
        }
    }
    println!("{}", res);
}
