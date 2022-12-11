use proconio::input;
use std::cmp::max;

fn main(){
    input!{
        n: usize,
        w: usize,
        tup: [(usize,usize);n],
    };
    let mut dp = vec![vec![0;100001];101];
    for i in 0..n{
        for j in 0..=w{
            if j >= tup[i].0 {
                dp[i+1][j] = max(dp[i+1][j],dp[i][j-tup[i].0] + tup[i].1);
            }

            dp[i+1][j] = max(dp[i+1][j],dp[i][j]);
        }
    }
    println!("{}",dp[n][w]);
}