use std::collections::BTreeMap;

use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize;n]
    };
    let mut map = BTreeMap::new();
    let mut v = vec![];
    let mut res: usize = 0;
    for e in a {
        if !map.contains_key(&e) {
            v.push(e);
        }
        *map.entry(e).or_insert(0) += 1;
    }
    v.sort();
    let l = v.len();
    for i in 0..l {
        for j in 0..l {
            if v[i] * v[j] > 200000 {
                break;
            }
            if let Some(&e) = map.get(&(v[i] * v[j])) {
                if let (Some(&e1), Some(&e2)) = (map.get(&v[i]), map.get(&v[j])) {
                    res += e1 * e2 * e;
                }
            }
        }
    }
    println!("{}", res);
}
