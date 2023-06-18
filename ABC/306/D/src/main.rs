use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, isize); n],
    }
    let mut dp = vec![vec![0_isize; n + 1]; 2];
    for (idx, (x, y)) in xy.into_iter().enumerate() {
        if x == 0 {
            dp[0][idx + 1] = dp[0][idx].max(dp[0][idx] + y).max(dp[1][idx] + y);
            dp[1][idx + 1] = dp[1][idx];
        } else {
            dp[0][idx + 1] = dp[0][idx];
            dp[1][idx + 1] = dp[1][idx].max(dp[0][idx] + y);
        }
    }
    println!("{}", dp[0][n].max(dp[1][n]));
}
