use proconio::*;

fn main() {
    input! {
        n: usize
    }
    let mut primes = vec![true; 2_000_001];
    let mut v = vec![];
    for i in 2..primes.len() {
        if !primes[i] {
            continue;
        }
        v.push(i as usize);
        for j in 2.. {
            if i * j >= primes.len() {
                break;
            }
            primes[i * j] = false;
        }
    }
    let mut res = 0;
    for &x in v.iter() {
        for &y in v.iter() {
            if x == y || x * x * y * y > n {
                break;
            }
            res += 1;
        }
        res += (x.saturating_pow(8) <= n) as i32;
    }
    println!("{}", res);
}
