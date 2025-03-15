use std::collections::HashMap;

use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize;n]
    };

    let mut dmap = HashMap::new();
    let mut pmap = HashMap::new();
    dmap.insert(a[0], 1);
    for i in 1..n {
        *pmap.entry(a[i]).or_insert(0) += 1;
    }
    let mut res = dmap.len() + pmap.len();
    for i in 1..n {
        if let Some(v) = pmap.get_mut(&a[i]) {
            *v -= 1;
            if *v == 0 {
                pmap.remove(&a[i]);
            }
        }

        *dmap.entry(a[i]).or_insert(0) += 1;
        res = res.max(dmap.len() + pmap.len());
    }
    println!("{}", res);
}
