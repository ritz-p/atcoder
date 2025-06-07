use std::collections::HashMap;

use proconio::input;
fn main() {
    input! {
        n: usize,
        l: usize,
        d: [usize;n-1]
    };
    if l % 3 != 0 {
        println!("0");
        return;
    }
    let mut current = 0;
    let mut map = HashMap::new();
    let mut res: usize = 0;
    *map.entry(current).or_insert(0) += 1;
    for i in 0..n - 1 {
        current = (current + d[i]) % l;
        *map.entry(current).or_insert(0) += 1;
    }

    for i in map.keys() {
        let j = (i + l / 3) % l;
        let k = (i + l / 3 * 2) % l;
        if let (Some(a), Some(b), Some(c)) = (map.get(&i), map.get(&j), map.get(&k)) {
            res += a * b * c;
        }
    }
    println!("{}", res / 3);
}
