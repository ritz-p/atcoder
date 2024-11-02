use proconio::input;
use std::collections::HashMap;
fn main() {
    input! {
        n: usize,
        a: [isize;n]
    };

    let mut map = HashMap::new();

    for i in 0..n {
        if !map.contains_key(&a[i]) {
            print!("-1 ");
            map.entry(a[i]).or_insert(i + 1);
        } else {
            if let Some(v) = map.get_mut(&a[i]) {
                print!("{} ", v);
                map.insert(a[i], i + 1);
            }
        }
    }
}
