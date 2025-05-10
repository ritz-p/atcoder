use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: [usize; n],
        t: [usize; n],
    }
    let mut res = t;

    for i in 0..n * 2 {
        res[(i + 1) % n] = (res[i % n] + s[i % n]).min(res[(i + 1) % n]);
    }

    println!("{}", res.iter().join("\n"));
}
