use proconio::input;

fn main(){
    input!{
        n: usize,
    };
    if n < 3{
        println!("0");
        return;
    }
    let mut dp = vec![1;n+1];
    dp[1] = 0;
    dp[2] = 0;
    for i in 3..=n{
        for j in 3..=n-3{
            if i + j > n{
                break;
            }
            dp[i+j] += dp[i];
            dp[i+j] %= 1000000007;
        }
    }
    println!("{}",dp[n]);
}
