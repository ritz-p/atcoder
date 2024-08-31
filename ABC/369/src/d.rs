use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n]
    };

    let mut dp = vec![(0, 0, 0); n + 1];
    dp[1].1 = a[0];
    for i in 1..n {
        dp[i + 1].0 = dp[i].0.max(dp[i].1).max(dp[i].2);

        dp[i + 1].1 = (dp[i].0.max(dp[i].2) + a[i]).max(dp[i].1);

        dp[i + 1].2 = dp[i].1 + 2 * a[i];
    }
    let res = dp[n].0.max(dp[n].1).max(dp[n].2);
    println!("{}", res);
}
