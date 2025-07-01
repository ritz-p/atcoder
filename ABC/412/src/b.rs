use std::{collections::HashSet, iter::FromIterator};

use proconio::{input, marker::Chars};
fn main() {
    input! {
        s: Chars,
        t: Chars,
    };
    let set: HashSet<char> = HashSet::from_iter(t);

    for (index, c) in s.iter().enumerate().skip(1) {
        if c.is_ascii_uppercase() {
            if !set.contains(&s[index - 1]) {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
