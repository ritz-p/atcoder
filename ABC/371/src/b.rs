use proconio::input;
use std::collections::HashSet;
fn main() {
    input! {
        _n: usize,
        m: usize,
        ab: [(usize,char);m]
    };
    let mut set = HashSet::new();
    for (a, b) in ab {
        if b == 'M' && !set.contains(&a) {
            set.insert(a);
            println!("Yes");
        } else {
            println!("No");
        }
    }
}
