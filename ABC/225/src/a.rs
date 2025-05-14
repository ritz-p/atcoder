use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    };
    let res = s.iter().permutations(s.len()).unique().count();
    println!("{}", res);
}
