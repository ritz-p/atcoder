use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        mut s: Chars,
        k: usize,
    };
    s.sort();
    for (index, cs) in s.iter().permutations(s.len()).unique().enumerate() {
        if index == k - 1 {
            println!("{}", cs.into_iter().collect::<String>());
            break;
        }
    }
}
