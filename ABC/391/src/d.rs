use std::collections::{HashMap, HashSet};

use proconio::input;

fn main() {
    input! {
        n: usize,
        w: usize,
        xy: [(usize,usize);n],
        q: usize,
        ta: [(usize,usize);q]
    };
    let mut map = HashMap::new();
    for (x, y) in xy {
        map.entry(y).or_insert(HashSet::new()).insert(x);
    }
    for (t, a) in ta {}
}
