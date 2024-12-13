use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        a: [isize;n+1],
        mut c: [isize;n + m+1]
    };
    let mut b = vec![0; m + 1];
    for i in (0..=m).rev() {
        b[i] = c[i + n] / a[n];
        for j in (0..=n).rev() {
            c[i + j] -= b[i] * a[j];
        }
    }

    println!("{}", b.iter().join(" "));
}
