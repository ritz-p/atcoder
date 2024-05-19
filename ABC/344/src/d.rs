use proconio::input;
use proconio::marker::Chars;
fn main(){
    input!{
        t: Chars,
        n: usize,
    };
    let mut s = Vec::new();
    for _ in 0..n{
        input!{
            a: usize,
            sa: [Chars;a]
        }
        s.push(sa);
    }
    let mut dp = vec![vec![usize::MAX-1;t.len()+1];n+1];
    dp[0][0] = 0;
    for i in 0..n{
        for j in 0..=t.len(){
            dp[i+1][j] = dp[i+1][j].min(dp[i][j]);
            for k in 0..s[i].len(){
                let l = s[i][k].len();
                if j + l <= t.len() && s[i][k] == t[j..j+l]{
                    dp[i+1][j+l] = dp[i+1][j+l].min(dp[i][j]+1);
                }
            }
        }
    }
    if dp[n][t.len()] == usize::MAX-1{
        println!("-1");
        return;
    }
    println!("{}",dp[n][t.len()]);
}
