use itertools::Itertools;
use proconio::{input, marker::Chars};

fn main() {
    input!(mut s: Chars);
    s.dedup();
    if "ABC".chars().powerset().contains(&s) {
        println!("Yes");
    } else {
        println!("No");
    }
}