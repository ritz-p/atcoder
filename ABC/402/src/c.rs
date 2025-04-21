use std::{collections::HashSet, iter::FromIterator};

use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
    };

    let mut f = vec![vec![]; n];
    let mut d: Vec<HashSet<usize>> = vec![];
    for i in 0..m {
        input! {
            k: usize,
            a: [usize;k]
        };
        for e in &a {
            f[e - 1].push(i);
        }
        d.push(HashSet::from_iter(a.iter().cloned()));
    }
    input! {
        b: [usize;n]
    };
    let mut res = 0;

    for e in &b {
        for i in &f[*e - 1] {
            d[*i].remove(e);
            if d[*i].len() == 0 {
                res += 1;
            }
        }
        println!("{}", res);
    }
}
