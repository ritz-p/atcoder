use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        _h: usize,
        _w: usize,
        n: usize,
        mut ab: [(usize,usize);n]
    };
    let mut r: Vec<usize> = ab.iter().map(|&(a, _)| a).collect();
    let mut c: Vec<usize> = ab.iter().map(|&(_, b)| b).collect();
    r.sort();
    c.sort();

    // 同じ列または行に並んでいる場合があるので dedup
    r.dedup();
    c.dedup();
    let rmap: BTreeMap<usize, usize> = r
        .into_iter()
        .enumerate()
        .map(|(i, x)| (x, i + 1)) // 1-indexed
        .collect();
    let cmap: BTreeMap<usize, usize> = c.into_iter().enumerate().map(|(i, x)| (x, i + 1)).collect();

    for (a, b) in ab {
        println!("{} {}", rmap[&a], cmap[&b]);
    }
}
