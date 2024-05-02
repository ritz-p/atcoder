use proconio::input;

fn main(){
    input!{
        n: usize,
        pa: [(usize,usize);n]
    };
    let mut dp = vec![vec![0;n];n];
    dp[1][n] = 0;
    for i in (0..n-2).rev(){
        for l in 1..=(n-i){
            let r = l + i;
            if l > 1{
                let s1 = if pa[l-1].0 >= l && pa[l-1].0 <= r{
                    pa[l-1].1
                }else{
                    0
                };
                dp[l][r] = dp[l][r].max(dp[l-1][r]+s1);
            }
            if r < n {
                let s2 = if pa[r + 1].0 >= l && pa[r + 1].0 <= r {
                    pa[r + 1].1
                } else {
                    0
                };
                dp[l][r] = dp[l][r].max(dp[l][r + 1] + s2);
            }
        }
    }
    let mut res = 0;
    for i in 0..n{
        res = res.max(dp[i][i]);
    }
    println!("{}",res);
}