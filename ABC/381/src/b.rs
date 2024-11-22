use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars
    };
    let l = s.len();
    if l % 2 != 0 {
        println!("No");
        return;
    }
    let mut set = HashSet::new();

    for i in 0..l {
        if i % 2 == 0 {
            if !set.contains(&s[i]) && s[i] == s[i + 1] {
                set.insert(s[i]);
            } else {
                println!("No");
                return;
            }
        }
    }
    println!("Yes");
}
