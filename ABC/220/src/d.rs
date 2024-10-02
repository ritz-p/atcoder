use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    };
    let m: usize = 998244353;
    let mut dp = vec![vec![0; 10]; n];
    dp[0][a[0]] = 1;
    for i in 1..n {
        for j in 0..=9 {
            dp[i - 1][j] %= m;
            dp[i][(j + a[i]) % 10] += dp[i - 1][j];
            dp[i][(j * a[i]) % 10] += dp[i - 1][j];
        }
    }
    for j in 0..=9 {
        println!("{}", dp[n - 1][j] % m);
    }
}
