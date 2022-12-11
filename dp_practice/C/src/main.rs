use proconio::input;
use std::cmp::max;
fn main(){
    input!{
        n: usize,
        arr: [[usize;3];n],
    };
    let mut dp = vec![vec![0;3];n+1];
    for i in 0..n{
        for j in 0..3{
            for k in 0..3{
                if j == k{
                    continue;
                }
                dp[i+1][k] = max(dp[i+1][k],dp[i][j] + arr[i][k]);
            }
        }
    }
    let mut res = 0;
    for i in 0..3{
        res = max(res,dp[n][i]);
    }
    println!("{}",res);
    // println!("{:?}",dp);
}