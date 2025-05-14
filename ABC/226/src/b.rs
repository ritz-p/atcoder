use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut set = HashSet::new();
    for _i in 0..n {
        input! {
            l: usize,
            a: [usize;l]
        };
        set.insert(a);
    }

    println!("{}", set.len());
}
