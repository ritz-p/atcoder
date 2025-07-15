use itertools::Itertools;
use proconio::{input, marker::Chars};
fn main() {
    input! {
        n: usize,
        k: usize,
        mut s: [Chars;n]
    };
    s = s.into_iter().take(k).collect::<Vec<Vec<_>>>();
    s.sort();

    println!(
        "{}",
        s.iter().map(|v| v.iter().collect::<String>()).join("\n")
    );
}
