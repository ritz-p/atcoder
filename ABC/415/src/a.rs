use std::{collections::HashSet, iter::FromIterator};

use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize;n],
        x: usize,
    };
    let set: HashSet<usize> = HashSet::from_iter(a);
    if set.contains(&x) {
        println!("Yes");
    } else {
        println!("No");
    }
}
