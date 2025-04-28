use std::collections::HashSet;

use proconio::input;
fn main() {
    input! {
        n: usize,
        _m: usize,
        q: usize,
    };
    let mut vset: Vec<HashSet<usize>> = vec![HashSet::new(); n];
    for _i in 0..q {
        input! {
            query: usize,
        };
        match query {
            1 => {
                input! {
                    x: usize,
                    y: usize,
                };
                vset[x - 1].insert(y);
            }
            2 => {
                input! {
                    x: usize,
                };
                vset[x - 1].insert(0);
            }
            3 => {
                input! {
                    x: usize,
                    y: usize,
                };
                if vset[x - 1].contains(&y) || vset[x - 1].contains(&0) {
                    println!("Yes");
                } else {
                    println!("No");
                }
            }
            _ => {}
        }
    }
}
