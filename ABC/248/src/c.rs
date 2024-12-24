use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        k: usize,
    };
    let modulo = 998244353;
    let mut dp = vec![vec![0; k + 1]; n + 1];
    dp[0][0] = 1;

    for i in 0..n {
        for j in 0..k {
            for ai in 1..=m {
                if ai + j <= k {
                    dp[i + 1][j + ai] = (dp[i + 1][j + ai] + dp[i][j]) % modulo;
                }
            }
        }
    }
    let mut res = 0;

    for i in 1..k + 1 {
        res = (res + dp[n][i]) % modulo;
    }

    println!("{}", res);
}
