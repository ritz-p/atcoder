use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    };
    let mut v = vec![0; n];
    for index in 0..n {
        v[a[index] - 1] = index + 1;
    }

    println!("{}", v.iter().join(" "));
}
