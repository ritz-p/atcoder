use proconio::input;
use std::collections::BTreeSet;
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize;n],
        b: [usize;m],
    };
    a.sort();
    let bmap: BTreeSet<usize> = b.into_iter().collect();

    let mut res = usize::MAX;

    for i in a {
        if let Some(v) = bmap.range(i..).next() {
            res = res.min(*v - i);
        }
        if let Some(v) = bmap.range(..=i).next_back() {
            res = res.min(i - *v);
        }
    }

    println!("{}", res);
}
