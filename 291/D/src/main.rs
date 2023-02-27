use proconio::input;

fn main() {
    input!{
        n: usize,
        tup: [[usize;2];n],
    };
    let m = 998244353;
    let mut dp = vec![vec![0,0];n+1];
    dp[0] = vec![1,1];
    for i in 1..n{
        for pre in 0..2{
            for next in 0..2{
                if tup[i-1][pre] != tup[i][next] {
                    dp[i][next] += dp[i-1][pre];
                }
            }
        }
        dp[i][0] %= m;
        dp[i][1] %= m;
    }


    println!("{}",(dp[n-1][0]+dp[n-1][1])%m);
}