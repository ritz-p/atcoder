use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {
        n: usize,
        a: [usize;n],
        b: [usize;n],
        c: [usize;n]
    };

    let mut map = HashMap::new();

    for i in c {
        *map.entry(b[i - 1]).or_insert(0) += 1;
    }

    let mut res: usize = 0;

    for e in &a {
        if let Some(v) = map.get(e) {
            res += v;
        }
    }

    println!("{}", res);
}
