use std::isize;

use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        lr: [(usize,usize);m]
    };
    let mut v = vec![0_isize; n + 1];

    for (l, r) in &lr {
        v[l - 1] += 1;
        v[*r] -= 1;
    }

    for i in 1..n {
        v[i] = v[i] + v[i - 1];
    }
    let mut res = isize::MAX;
    for i in 0..n {
        res = res.min(v[i]);
    }

    println!("{}", res);
}
