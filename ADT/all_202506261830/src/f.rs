use std::collections::HashMap;

use proconio::input;
fn main() {
    input! {
        n: usize,
        a: [usize;n]
    };

    let mut lmap = HashMap::new();
    let mut rmap = HashMap::new();
    lmap.insert(a[0], 1);
    for i in 1..n {
        *rmap.entry(a[i]).or_insert(0) += 1;
    }
    let mut res = lmap.len() + rmap.len();

    for i in 1..n - 1 {
        *lmap.entry(a[i]).or_insert(0) += 1;
        if let Some(v) = rmap.get_mut(&a[i]) {
            *v -= 1;
            if *v == 0 {
                rmap.remove(&a[i]);
            }
        }
        res = res.max(lmap.len() + rmap.len());
    }

    println!("{}", res);
}
