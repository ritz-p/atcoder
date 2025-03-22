use std::collections::BTreeSet;

use proconio::{input, marker::Chars};
fn main() {
    input! {
        _n: usize,
        r: isize,
        c: isize,
        s: Chars
    };
    let mut set = BTreeSet::new();
    set.insert((0, 0));
    let mut rpos = 0;
    let mut cpos = 0;

    for ch in s {
        match ch {
            'N' => rpos -= 1,
            'S' => rpos += 1,
            'W' => cpos -= 1,
            'E' => cpos += 1,
            _ => {}
        }
        set.insert((rpos, cpos));
        let q = (rpos - r, cpos - c);
        if set.contains(&q) {
            print!("1");
        } else {
            print!("0");
        }
    }
    println!();
}
