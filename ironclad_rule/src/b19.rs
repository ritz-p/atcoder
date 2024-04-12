use proconio::input;

fn main(){
    input!{
        n: usize,
        w: usize,
        mut _tup: [(usize,usize);n]
    };
    let mut dp = vec![vec![1000000001;100001];n+1];
    let mut tup = vec![(0,0)];
    tup.append(&mut _tup);
    dp[0][0] = 0;

    for i in 1..=n{
        for j in 0..=100000{
            if j < tup[i].1{
                dp[i][j] = dp[i-1][j];
            }else{
                dp[i][j] = dp[i-1][j].min(dp[i-1][j-tup[i].1] + tup[i].0);
            }
        }
    }
    let mut res = 0;
    for i in 0..=100000{
        if dp[n][i] <= w{
            res = i;
        }
    }
    println!("{}",res);
}