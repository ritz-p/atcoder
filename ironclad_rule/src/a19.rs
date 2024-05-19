use proconio::input;

fn main(){
    input!{
        n: usize,
        w: usize,
        tup: [(usize,usize);n]
    };
    let mut dp = vec![vec![0;w+1];n+1];
    for i in 1..=n{
        for j in 0..=w{
            if j >= tup[i-1].0{
                dp[i][j] = dp[i-1][j].max(dp[i-1][j-tup[i-1].0] + tup[i-1].1);
            }else{
                dp[i][j] = dp[i-1][j];
            }
        }
    }
    let mut res = 0;
    for i in 0..=n{
        res = res.max(dp[i][w]);
    }
    println!("{}",res);
}