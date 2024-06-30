use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {
        n: usize,
        a: [usize;n],
        w: [usize;n]
    };
    let mut map = HashMap::new();
    let mut res = 0;
    for i in 0..n {
        if map.contains_key(&a[i]) {
            if let Some(v) = map.get(&a[i]) {
                if v >= &w[i] {
                    res += w[i];
                } else {
                    res += v;
                    map.insert(a[i], w[i]);
                }
            }
        } else {
            map.insert(a[i], w[i]);
        }
    }
    println!("{}", res);
}
