use std::collections::BTreeMap;

use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        t: usize,
    };
    let mut res = vec![];
    for _i in 0..t {
        input! {
            n: usize,
            m: usize,
            a: [usize;n],
            mut b: [usize;n],
        }
        let mut current = 0;
        b.sort();
        let mut bmap = BTreeMap::new();
        for e in a {
            *bmap.entry(e).or_insert(0) += 1;
        }
        for e in b.iter().rev() {
            if let Some((k, &v)) = bmap.range(m - e..).next() {
                current += (k + e) % m;
                let key = *k;
                if v > 1 {
                    bmap.insert(key, v - 1);
                } else {
                    bmap.remove(&key);
                }
            } else {
                if let Some((k, v)) = bmap.pop_first() {
                    current += k + e;

                    if v > 1 {
                        bmap.insert(k, v - 1);
                    }
                }
            }
        }
        res.push(current);
    }
    println!("{}", res.iter().join("\n"));
}
