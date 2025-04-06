use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        d: usize,
        n: usize,
        mut lr: [(usize,usize);n]
    };
    let mut plus = vec![0; d + 1];
    let mut minus = vec![0; d + 2];
    for (l, r) in &lr {
        plus[*l] += 1;
        minus[*r + 1] += 1;
    }
    let mut sums = vec![0; d + 1];
    for i in 0..d {
        sums[i + 1] += sums[i] + plus[i + 1] - minus[i + 1];
    }
    println!("{}", sums.iter().skip(1).join("\n"));
}
