use proconio::*;
use std::collections::BTreeMap;

fn main() {
    input! { n: usize, a: usize, b: usize, mut v: [usize; n] }
    v.sort_by_key(|&x| std::cmp::Reverse(x));
    let mut map = BTreeMap::new();
    for &v in &v {
        *map.entry(v).or_insert(0) += 1;
    }
    println!("{:.6}", v[..a].iter().sum::<usize>() as f64 / a as f64);
    let c = *map.get(&v[0]).unwrap();
    if c >= a {
        let mut ans = 0;
        for i in a..=b.min(c) {
            ans += comb(c, i);
        }
        println!("{ans}");
    } else {
        let mut cnt = 0;
        for i in 1.. {
            if v[a - i] != v[a - 1] {
                break;
            }
            cnt += 1;
        }
        let c = *map.get(&(v[a - 1])).unwrap();
        println!("{}", comb(c, cnt));
    }
}

//ncr
fn comb(n: usize, r: usize) -> usize {
    assert!(n >= r);
    if r == 0 || n == r {
        return 1;
    }
    return comb(n - 1, r) * n / (n - r);
}
