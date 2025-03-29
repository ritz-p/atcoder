use std::collections::{HashMap, HashSet};

use proconio::input;
fn main() {
    input! {
        t: usize
    };
    let mut ns = vec![];
    let mut avs = vec![];
    for _i in 0..t {
        input! {
            n: usize,
            a: [usize;2*n]
        };
        ns.push(n);
        avs.push(a);
    }

    for i in 0..t {
        let mut res = HashSet::new();
        let n = ns[i];
        let a = &avs[i];
        let mut map = HashMap::new();
        for i in 0..2 * n {
            map.entry(a[i]).or_insert(vec![]).push(i);
        }

        for j in 0..2 * n - 1 {
            if let (Some(f), Some(s)) = (map.get(&a[j]), map.get(&a[j + 1])) {
                if f[0] + 1 == f[1] || s[0] + 1 == s[1] {
                    continue;
                }
                let mut v = vec![f[0], f[1], s[0], s[1]];
                v.sort();
                if v[0] + 1 == v[1] && v[2] + 1 == v[3] {
                    res.insert((f.min(s), f.max(s)));
                }
            }
        }
        println!("{}", res.len());
    }
}
