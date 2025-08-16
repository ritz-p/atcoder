use std::collections::BTreeMap;

use proconio::input;
fn main() {
    input! {
        q: usize,
    };
    let mut bmap = BTreeMap::new();
    for _i in 0..q {
        input! {
            qt: usize,
        };
        match qt {
            1 => {
                input! {
                    x: usize,
                };
                *bmap.entry(x).or_insert(0) += 1;
            }
            2 => {
                if let Some((k, v)) = bmap.pop_first() {
                    println!("{}", k);
                    if v > 0 {
                        bmap.insert(k, v - 1);
                    }
                }
            }
            _ => {}
        }
    }
}
