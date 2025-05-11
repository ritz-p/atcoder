use std::{collections::HashSet, iter::FromIterator};

use proconio::input;

fn main() {
    input! {
        s: [String;3]
    };
    let set: HashSet<String> = HashSet::from_iter(s);
    let c = vec!["ABC", "ARC", "AGC", "AHC"];
    for cs in c {
        if !set.contains(cs) {
            println!("{}", cs);
            return;
        }
    }
}
