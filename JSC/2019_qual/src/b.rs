use proconio::input;
fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize;n],
    };
    let mut ac = a.clone();
    ac.sort();
    let mut res = 0;
    let m = 1000000007;
    for i in 0..n {
        for j in 0..n {
            if a[i] > a[j] {
                res += if i <= j {
                    (k + 1) * k / 2 % m
                } else {
                    k * (k - 1) / 2 % m
                };
                res %= m;
            }
        }
    }
    println!("{}", res);
}
