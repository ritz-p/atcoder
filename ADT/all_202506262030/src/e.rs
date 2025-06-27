use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        n: usize,
        p: [usize;n]
    };
    let mut res = vec![0; n];

    for i in 0..n {
        res[p[i] - 1] = i + 1;
    }

    println!("{}", res.iter().join(" "));
}
