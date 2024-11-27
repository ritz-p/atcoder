use std::collections::BTreeMap;

use proconio::input;

fn main() {
    input! {
        q: usize,
    };
    let mut bmap = BTreeMap::new();
    for _i in 0..q {
        input! {
            c: usize,
            x: usize,
        };
        if c == 1 {
            *bmap.entry(x).or_insert(0) += 1;
            continue;
        }
        input! {
            k: usize
        };
        let mut current = 0;
        if c == 2 {
            // while let Some() = map.range は一回変数化の必要がある
            let mut iter = bmap.range(..=x).rev();
            while let Some((key, value)) = iter.next() {
                current += value;
                if current >= k {
                    println!("{}", key);
                    break;
                }
            }

            if current < k {
                println!("-1");
            }
        } else {
            let mut iter = bmap.range(x..);
            while let Some((key, value)) = iter.next() {
                current += value;
                if current >= k {
                    println!("{}", key);
                    break;
                }
            }
            if current < k {
                println!("-1");
            }
        }
    }
}
