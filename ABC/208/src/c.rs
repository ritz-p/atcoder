use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        n: usize,
        mut k: usize,
        a: [usize;n]
    };
    let mut v = vec![];
    for i in 0..n {
        v.push((a[i], i));
    }
    v.sort();
    let mut res = vec![k/n;n];
    k %= n;
    for i in 0..k {
        res[v[i].1] += 1;
    }
    println!("{}",res.iter().join("\n"));
}
