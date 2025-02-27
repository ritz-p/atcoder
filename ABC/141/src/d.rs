use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize;n]
    };
    let mut bmap = BTreeMap::new();

    for e in a {
        *bmap.entry(e).or_insert(0) += 1;
    }
    for _i in 0..m {
        if let Some((&key, _)) = bmap.iter_mut().next_back() {
            *bmap.entry(key / 2).or_insert(0) += 1;
            if let Some(v) = bmap.get_mut(&key) {
                *v -= 1;
                if *v == 0 {
                    bmap.remove(&key);
                }
            }
        }
    }
    let mut res = 0;
    for (k, v) in bmap {
        res += k * v;
    }

    println!("{}", res);
}
