use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {
        n: usize,
        k: usize,
        c: [usize;n]
    };

    let mut map = HashMap::new();
    for i in 0..k {
        *map.entry(c[i]).or_insert(0) += 1;
    }
    let mut res = map.len();

    for i in k..n {
        *map.entry(c[i]).or_insert(0) += 1;

        if let Some(value) = map.get_mut(&c[i - k]) {
            *value -= 1;
            if *value == 0 {
                map.remove(&c[i - k]);
            }
        }
        res = map.len().max(res);
    }

    println!("{}", res);
}
