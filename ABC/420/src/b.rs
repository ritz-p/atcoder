use std::collections::HashSet;

use itertools::Itertools;
use proconio::{input, marker::Chars};
fn main() {
    input! {
        n: usize,
        m: usize,
        ss: [Chars;n]
    };
    let mut zv = vec![HashSet::new(); m];
    let mut ov = vec![HashSet::new(); m];

    for (si, s) in ss.iter().enumerate() {
        for (ci, c) in s.iter().enumerate() {
            if *c == '0' {
                zv[ci].insert(si);
            } else {
                ov[ci].insert(si);
            }
        }
    }
    let mut v = vec![0; n];
    for i in 0..m {
        if zv[i].len() < ov[i].len() && zv[i].len() > 0 {
            for c in &zv[i] {
                v[*c] += 1;
            }
        } else if ov[i].len() < zv[i].len() && ov[i].len() > 0 {
            for c in &ov[i] {
                v[*c] += 1;
            }
        }
    }
    let mut max = 0;
    for i in 0..n {
        max = max.max(v[i]);
    }
    let mut res = vec![];
    for i in 0..n {
        if v[i] == max {
            res.push(i + 1);
        }
    }
    res.sort();
    println!("{}", res.iter().join(" "));
}
