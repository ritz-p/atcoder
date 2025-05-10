use proconio::input;
use std::{collections::BTreeSet, iter::FromIterator, usize};
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [isize;n],
        b: [isize;m]
    };
    let bset: BTreeSet<isize> = BTreeSet::from_iter(b);

    let mut res = isize::MAX;
    for e in a {
        if let Some(v) = bset.range(e..).next() {
            res = res.min(v - e);
        }
        if let Some(v) = bset.range(..=e).next_back() {
            res = res.min(e - v);
        }
    }
    println!("{}", res);
}
