use proconio::input;
fn main() {
    input! {
        n: usize,
    };

    let mut b = 1;
    let m = 998244353;
    let mut res = (n % m) * ((n + 1) % m) / 2;

    while b * b <= n {
        let current = n / b;
        res -= ((((current % m) - b % m) * 2) % m + 1) % m;
        b += 1;
    }

    println!("{}", res % m);
}
