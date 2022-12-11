use proconio::input;
use std::cmp::max;
fn main(){
    input!{
        n:usize,
        k:usize,
        d:usize,
        mut arr:[usize;n],
    };
    let mut dp:Vec<Vec<Vec<isize>>> = vec![vec![vec![-1;d];k+1];n+1];
    // println!("{:?}",dp);
    dp[0][0][0] = 0;
    for i in 0..n{
        for j in 0..=k{
            for l in 0..d{
                if dp[i][j][l] == -1{
                    continue;
                }
                dp[i+1][j][l] = max(dp[i][j][l],dp[i+1][j][l]);
                // println!("{}",dp[i+1][j][l]);
                // println!("{}",l+arr[i]);
                if j < k{
                    dp[i+1][j+1][(l+arr[i])%d] = max(dp[i+1][j+1][(l+arr[i])%d],dp[i][j][l]+(arr[i] as isize));
                }
            }
        }
    }
    println!("{}",dp[n][k][0]);
}