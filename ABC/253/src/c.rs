use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        q: usize,
    };
    let mut bmap = BTreeMap::new();

    for _i in 0..q {
        input! {
            query: usize,
        };
        match query {
            1 => {
                input! {
                    x: usize
                };
                *bmap.entry(x).or_insert(0) += 1;
            }
            2 => {
                input! {
                    x: usize,
                    c: usize,
                };
                if let Some(v) = bmap.get_mut(&x) {
                    if *v > c {
                        *v -= c;
                    } else {
                        *v = 0;
                    }
                    if *v == 0 {
                        bmap.remove(&x);
                    }
                }
            }
            3 => {
                if let Some((max, _)) = bmap.last_key_value() {
                    if let Some((min, _)) = bmap.first_key_value() {
                        println!("{}", max - min);
                    }
                }
            }
            _ => {}
        }
    }
}
