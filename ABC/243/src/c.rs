use std::collections::BTreeMap;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        xy: [(usize,usize);n],
        s: Chars
    };
    let mut bmap_left = BTreeMap::new();
    let mut bmap_right = BTreeMap::new();
    for i in 0..n {
        let (x, y) = xy[i];
        match s[i] {
            'L' => {
                bmap_left
                    .entry(y)
                    .and_modify(|e| *e = x.max(*e))
                    .or_insert(x);
            }
            'R' => {
                bmap_right
                    .entry(y)
                    .and_modify(|e| *e = x.min(*e))
                    .or_insert(x);
            }
            _ => {}
        }
    }
    for (key, value) in bmap_right {
        if bmap_left.get(&key).is_some() && bmap_left[&key] >= value {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
