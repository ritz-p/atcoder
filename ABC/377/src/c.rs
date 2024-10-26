use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize,usize);m]
    };
    let mut set = HashSet::new();

    for (mut a, mut b) in ab {
        a = a - 1;
        b = b - 1;
        set.insert((a, b));
        if a < n - 2 && b < n - 1 {
            set.insert((a + 2, b + 1));
        }
        if a < n - 1 && b < n - 2 {
            set.insert((a + 1, b + 2));
        }
        if a > 0 && b < n - 2 {
            set.insert((a - 1, b + 2));
        }
        if a > 1 && b < n - 1 {
            set.insert((a - 2, b + 1));
        }
        if a > 1 && b > 0 {
            set.insert((a - 2, b - 1));
        }
        if a > 0 && b > 1 {
            set.insert((a - 1, b - 2));
        }
        if a < n - 1 && b > 1 {
            set.insert((a + 1, b - 2));
        }
        if a < n - 2 && b > 0 {
            set.insert((a + 2, b - 1));
        }
    }
    println!("{}", (n * n) as usize - set.len());
}
