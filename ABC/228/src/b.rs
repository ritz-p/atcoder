use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize;n]
    };
    let mut set = HashSet::new();

    let mut current = x - 1;
    set.insert(x - 1);
    while !set.contains(&(a[current] - 1)) {
        set.insert(a[current] - 1);
        current = a[current] - 1;
    }

    println!("{}", set.len());
}
