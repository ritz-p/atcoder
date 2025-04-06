use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
        lr: [(usize,usize);n]
    };
    let mut sums = vec![0; t + 1];
    let mut plus = vec![0; t];
    let mut minus = vec![0; t + 1];
    for (l, r) in lr {
        plus[l] += 1;
        minus[r] += 1;
    }
    for i in 0..t {
        sums[i + 1] += plus[i] - minus[i] + sums[i];
    }

    println!("{}", sums.iter().skip(1).join("\n"));
}
