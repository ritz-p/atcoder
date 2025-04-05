use std::collections::BTreeSet;

use itertools::Itertools;
use proconio::{input, marker::Chars};
fn main() {
    input! {
        s: Chars
    };
    let l = s.len();
    let mut res = vec![0; l];
    let mut lset = BTreeSet::new();
    let mut rset = BTreeSet::new();
    for (index, c) in s.iter().enumerate() {
        match c {
            'L' => {
                lset.insert(index);
            }
            'R' => {
                rset.insert(index);
            }
            _ => {}
        }
    }
    for (index, c) in s.iter().enumerate() {
        match c {
            'L' => {
                if let Some(v) = rset.range(..index).next_back() {
                    if (index - v) % 2 == 1 {
                        res[*v + 1] += 1;
                    } else {
                        res[*v] += 1;
                    }
                }
            }
            'R' => {
                if let Some(v) = lset.range(index + 1..).next() {
                    if (v - index) % 2 == 1 {
                        res[*v - 1] += 1;
                    } else {
                        res[*v] += 1;
                    }
                }
            }
            _ => {}
        }
    }
    println!("{}", res.iter().join(" "));
}
