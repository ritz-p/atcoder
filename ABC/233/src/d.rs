use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: isize,
        a: [isize;n]
    };
    let mut sums = vec![0;n+1];
    for i in 0..n{
        sums[i+1] = sums[i] + a[i];
    }

    let mut map: HashMap<isize,usize> = HashMap::new();
    let mut res = 0;
    for i in 1..=n{
        *map.entry(sums[i-1]).or_insert(0) += 1;
        if let Some(v) = map.get(&(sums[i] - k)){
            res += v;
        }
    }
    println!("{}",res);
}
