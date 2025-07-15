use std::collections::HashMap;

use itertools::Itertools;
use proconio::input;
fn main() {
    input! {
        q: usize,
    };
    let mut map = HashMap::new();
    let mut res = vec![];
    for _ in 0..q {
        input! {
            query: usize,
        };
        match query {
            1 => {
                input! {
                    x: usize,
                };
                *map.entry(x).or_insert(0) += 1;
            }
            2 => {
                input! {
                    x: usize,
                };
                if let Some(v) = map.get_mut(&x) {
                    *v -= 1;
                    if *v == 0 {
                        map.remove(&x);
                    }
                }
            }
            3 => {
                res.push(map.len());
            }
            _ => {}
        }
    }
    println!("{}", res.iter().join("\n"));
}
