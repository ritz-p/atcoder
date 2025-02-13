use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let mut dice = vec![];
    let mut ks = vec![];
    for _i in 0..n {
        input! {
            k: usize,
            a: [usize;k]
        };
        ks.push(k);
        let mut map = HashMap::new();
        for e in a {
            *map.entry(e).or_insert(0 as usize) += 1;
        }
        dice.push(map);
    }
    let mut res: f64 = 0.0;
    for i in 0..n {
        for j in i + 1..n {
            if i == j {
                continue;
            }
            let mut count = 0;
            for (k, v) in &dice[i] {
                if let Some(v2) = dice[j].get(k) {
                    count += v * v2;
                }
            }

            res = res.max(count as f64 / (ks[i] * ks[j]) as f64);
        }
    }
    println!("{:.9}", res);
}
