use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [String;n],
        t: [String;m]
    };

    let mut set = HashSet::new();
    for st in t {
        set.insert(st);
    }
    for ss in s {
        if set.contains(&ss) {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
