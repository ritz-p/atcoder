use std::collections::HashSet;

use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        mut a: [usize;n]
    };
    let mut set = HashSet::new();
    for e in 1..=m {
        set.insert(e);
    }
    let mut res = 0;
    loop {
        let mut v = vec![false; m];
        for e in &a {
            v[*e - 1] = true;
        }
        if v.iter().all(|f| *f == true) {
            a.pop();
            res += 1;
        } else {
            break;
        }
    }
    println!("{}", res);
}
