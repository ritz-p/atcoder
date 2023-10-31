use proconio::input;
use proconio::marker::Chars;
fn main() {
    input!{
        n: usize,
        s: Chars
    };
    let atcoder:Vec<char> = vec!['a','t','c','o','d','e','r'];
    let mut dp = vec![vec![0;atcoder.len()+1];n+1];
    let m = 1000000007;
    dp[0][0] = 1;
    for i in 0..n{
        for j in 0..=atcoder.len(){
            dp[i+1][j] += dp[i][j];
            if dp[i+1][j] >= m{
                dp[i+1][j] -= m;
            }
            if j < atcoder.len() && s[i] == atcoder[j]{
                dp[i+1][j+1] += dp[i][j];
                if dp[i+1][j+1] >= m{
                    dp[i+1][j+1] -= m;
                }
            }
        }
    }
    println!("{}",dp[n][atcoder.len()]);
}