use proconio::input;

fn main(){
    input!{
        n: usize,
        w: usize,
        tup: [(usize,usize);n]
    };
    let mut dp = vec![vec![-1;w+1];n+1];
    dp[0][0] = 0;

    for i in 0..n{
        for j in 0..=w{
            if dp[i][j] != -1{
                dp[i+1][j] = dp[i+1][j].max(dp[i][j]);
                if j + tup[i].0 <= w{
                    dp[i+1][j + tup[i].0] = dp[i+1][j+tup[i].0].max(dp[i][j] + tup[i].1 as isize);
                }
            }
        }
    }
    let mut res = 0;
    for i in 0..=w{
        res = res.max(dp[n][i]);
    }
    println!("{}",res);
}