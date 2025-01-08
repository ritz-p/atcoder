use itertools::Itertools;
use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n: usize,
        q: usize,
        x: [usize;q]
    };
    let mut v = vec![];
    let mut map = HashMap::new();
    for i in 0..n {
        v.push(i + 1);
        map.insert(i + 1, i);
    }
    for e in x {
        if let Some(&left) = map.get(&e) {
            if left >= n - 1 {
                let tmp = v[left];
                v[left] = v[left - 1];
                v[left - 1] = tmp;
                map.insert(v[left], left - 1);
                map.insert(v[left - 1], left - 1);
            } else {
                let tmp = v[left];
                v[left] = v[left + 1];
                v[left + 1] = tmp;
                map.insert(v[left], left);
                map.insert(v[left + 1], left + 1);
            }
        }
    }
    println!("{}", v.iter().join(" "));
}
