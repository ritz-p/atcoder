use std::{collections::BTreeMap, usize};

use proconio::input;
fn main() {
    input! {
        n: usize,
        mut a: [usize;n]
    };
    a.sort();
    let mut bmap = BTreeMap::new();
    for i in 0..n {
        if bmap.contains_key(&a[i]) {
            *bmap.entry(usize::MAX).or_insert(0) += 1;
        } else {
            bmap.insert(a[i], 1);
        }
    }
    let mut res = 0;
    for i in 1..=n {
        if !bmap.contains_key(&i) {
            let mut current = 0;
            if let Some((k, v)) = bmap.pop_last() {
                if i > k {
                    break;
                }
                if v > 2 {
                    current = 2;
                    bmap.insert(k, v - 2);
                } else if v == 2 {
                    current = 2;
                } else {
                    current += 1;
                    if let Some((k2, v2)) = bmap.pop_last() {
                        if i > k2 {
                            break;
                        }
                        if v2 > 1 {
                            bmap.insert(k2, v2 - 1);
                        }
                        current += 1;
                    }
                }
            }
            if current == 2 {
                res += 1;
            } else {
                break;
            }
        } else {
            res += 1;
        }
    }
    println!("{}", res);
}
