use proconio::input;

fn main(){
    input!{
        n: usize,
        w: usize,
        mut wv: [(usize,usize);n]
    };
    let sum:usize = wv.iter().map(|(_a,b)|b).sum();
    let mut dp = vec![vec![1000000001;sum+1];n+1];
    dp[0][0] = 0;

    for i in 1..=n{
        for j in 0..=sum{
            if j < wv[i-1].1{
                dp[i][j] = dp[i-1][j];
            }else{
                dp[i][j] = (dp[i-1][j-wv[i-1].1] + wv[i-1].0).min(dp[i-1][j]);
            }
        }
    }
    let mut res = 0;
    for i in 0..=sum{
        if dp[n][i] <= w{
            res = i;
        }
    }
    println!("{}",res);
}