use proconio::input;
use std::collections::BTreeMap;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    };
    let mut bmap = BTreeMap::new();
    for e in &a {
        *bmap.entry(e).or_insert(0) += 1;
    }
    let mut res = 0;
    for e in &a {
        let Some(count) = bmap.get_mut(&e) else {
            continue;
        };
        if *count == 0 {
            continue;
        }
        *count -= 1;
        if *count == 0 {
            bmap.remove(&e);
        }
        let remove_key = if let Some((k, v)) = bmap.range_mut(e * 2..).next() {
            res += 1;
            *v -= 1;
            if *v == 0 {
                Some(*k)
            } else {
                None
            }
        } else {
            None
        };
        if let Some(k) = remove_key {
            bmap.remove(&k);
        }
    }

    println!("{}", res);
}
