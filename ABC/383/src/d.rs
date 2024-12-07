use proconio::input;

fn main() {
    input! {
        n: usize,
    };
    let primes = primes(n);

    let mut res = 0;
    for &p in &primes {
        if p * p * p * p * p * p * p * p > n {
            break;
        }
        res += 1;
    }
    for i in 0..primes.len() {
        let p = primes[i];
        if p * p > n {
            break;
        }
        for j in i + 1..primes.len() {
            let q = primes[j];
            if p * p * q > n {
                break;
            }
            res += 1;
        }
    }
    println!("{}", res);
}
fn primes(limit: usize) -> Vec<usize> {
    let mut is_prime = vec![true; limit + 1];
    let mut primes = Vec::new();

    is_prime[0] = false;
    if limit >= 1 {
        is_prime[1] = false;
    }

    for i in 2..=limit {
        if is_prime[i] {
            primes.push(i);
            for j in (i * i..=limit).step_by(i) {
                is_prime[j] = false;
            }
        }
    }

    primes
}
