use std::collections::{BTreeMap, HashMap};

use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize;n]
    };
    let mut bmap = BTreeMap::new();
    let mut map = HashMap::new();
    for (index, e) in a.iter().enumerate() {
        *bmap.entry(e).or_insert(0) += 1;
        map.insert(e, index);
    }

    while let Some((k, v)) = bmap.pop_last() {
        if v == 1 {
            if let Some(index) = map.get(&k) {
                println!("{}", index + 1);
            }
            return;
        }
    }

    println!("-1");
}
