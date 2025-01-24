use std::collections::HashMap;

use proconio::input;
fn main() {
    input! {
        n: usize,
    };
    let mut spf = vec![0; n + 1];
    for i in 2..=n {
        if spf[i] == 0 {
            spf[i] = i;
            let mut j = i * 2;
            while j <= n {
                if spf[j] == 0 {
                    spf[j] = i;
                }
                j += i;
            }
        }
    }
    let mut sf = vec![0; n + 1];
    sf[1] = 1;
    for i in 2..=n {
        let p = spf[i];
        let x = i / p;
        if x % p == 0 {
            sf[i] = sf[i / (p * p)];
        } else {
            sf[i] = p * sf[x];
        }
    }
    let mut count_map = HashMap::new();
    for i in 1..=n {
        *count_map.entry(sf[i]).or_insert(0usize) += 1;
    }

    let mut res = 0;
    for &count in count_map.values() {
        res += count * count;
    }

    println!("{}", res);
}
