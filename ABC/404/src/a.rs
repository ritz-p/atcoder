use std::{collections::HashSet, iter::FromIterator};

use proconio::{input, marker::Chars};
fn main() {
    input! {
        s: Chars,
    };
    let alphabet = "abcdefghijklmnopqrstuvwxyz";
    let set: HashSet<char> = HashSet::from_iter(s);

    for c in alphabet.chars() {
        if !set.contains(&c) {
            println!("{}", c);
            return;
        }
    }
}
