use std::collections::HashMap;

use proconio::input;
fn main() {
    input! {
        n: usize,
        mut a: [usize;n]
    };
    let mut map = HashMap::new();
    let mut res: usize = 0;

    for (i, &v) in a.iter().enumerate() {
        let key = i as isize - v as isize;
        if let Some(&count) = map.get(&key) {
            res += count;
        }

        let key = i as isize + v as isize;
        *map.entry(key).or_insert(0) += 1;
    }

    println!("{}", res);
}
