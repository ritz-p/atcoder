use std::collections::HashMap;

use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        c: [String;n],
        d: [String;m],
        p: [usize;m+1]
    };
    let mut map = HashMap::new();

    for i in 0..m {
        map.insert(d[i].clone(), p[i + 1]);
    }
    let mut res = 0;

    for e in c {
        if let Some(v) = map.get(&e) {
            res += v;
        } else {
            res += p[0];
        }
    }
    println!("{}", res);
}
