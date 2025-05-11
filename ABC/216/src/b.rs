use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        st: [(String,String);n]
    };

    let mut set = HashSet::new();
    for (s, t) in &st {
        if !set.contains(&(s, t)) {
            set.insert((s, t));
        } else {
            println!("Yes");
            return;
        }
    }
    println!("No");
}
