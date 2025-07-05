use std::collections::HashSet;

use proconio::input;
fn main() {
    input! {
        n: usize,
        s: [String;n]
    };
    let mut set = HashSet::new();

    for i in 0..n {
        for j in 0..n {
            if i == j {
                continue;
            }
            let k = s[i].clone() + &s[j];
            set.insert(k);
        }
    }
    println!("{}", set.len());
}
