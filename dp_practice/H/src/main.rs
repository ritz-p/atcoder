use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        h: usize,
        w: usize,
        arr: [Chars;h],
    };
    let mut dp = vec![vec![0;w];h];
    let mod_num = 1000000007;
    dp[0][0] = 1;
    for i in 0..h{
        for j in 0..w{
            if arr[i][j] == '#'{
                continue;
            }
            if i > 0{
                dp[i][j] += dp[i-1][j]
            }
            if j > 0{
                dp[i][j] += dp[i][j-1];
            }
            dp[i][j] %= mod_num;
        }
    }
    println!("{}",dp[h-1][w-1]);
}