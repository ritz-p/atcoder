use proconio::input;

fn main(){
    input!{
        n: usize,
        a: [usize;n]
    };

    let mut dp = vec![vec![0;n+1];n+1];
    for i in 1..=n{
        dp[n][i] = a[i-1];
    }
    for i in (1..n).rev(){
        for j in 1..n{
            if i % 2 == 1{
                dp[i][j] = dp[i+1][j].max(dp[i+1][j+1]);
            }else{
                dp[i][j] = dp[i+1][j].min(dp[i+1][j+1]);
            }
        }
    }
    println!("{}",dp[1][1]);
}