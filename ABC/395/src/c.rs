use proconio::input;
use std::{collections::HashMap, usize};
fn main() {
    input! {
        n: usize,
        a: [usize;n]
    };
    let mut res = usize::MAX;
    let mut map = HashMap::new();
    for (index, e) in a.iter().enumerate() {
        if map.contains_key(e) {
            if let Some(v) = map.get(&e) {
                res = res.min(index - v + 1);
                map.insert(e, index);
            }
        } else {
            map.insert(e, index);
        }
    }
    if res == usize::MAX {
        println!("-1");
    } else {
        println!("{}", res);
    }
}
