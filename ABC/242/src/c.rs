use proconio::input;

const MOD: usize = 998244353;

fn main() {
    input! {
        n: usize
    };
    let mut dp = vec![vec![0, 0, 0, 0, 0, 0, 0, 0, 0]; n];
    dp[0] = vec![1, 1, 1, 1, 1, 1, 1, 1, 1];
    for i in 1..n {
        for j in 0..9 {
            dp[i][j] += dp[i - 1][j] % MOD;
            if j == 0 {
                dp[i][j + 1] += dp[i - 1][j] % MOD;
            } else if j == 8 {
                dp[i][j - 1] += dp[i - 1][j] % MOD;
            } else {
                dp[i][j + 1] += dp[i - 1][j] % MOD;
                dp[i][j - 1] += dp[i - 1][j] % MOD;
            }
        }
    }
    let res = dp[n - 1].iter().sum::<usize>() % MOD;
    println!("{}", res);
}
