use std::{collections::HashSet, iter::FromIterator};

use proconio::{input, marker::Chars};
fn main() {
    input! {
        s: Chars,
    };
    let set: HashSet<char> = HashSet::from_iter(s);
    if set.contains(&'A') && set.contains(&'B') && set.contains(&'C') {
        println!("Yes");
    } else {
        println!("No");
    }
}
