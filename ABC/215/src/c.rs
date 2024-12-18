use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        mut s: Chars,
        k: usize,
    };
    s.sort();
    let permutation = s.iter().permutations(s.len()).unique().collect::<Vec<_>>();
    println!("{}", permutation[k - 1].iter().join(""));
}
