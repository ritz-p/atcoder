use proconio::input;

fn main(){
    input!{
        n: usize,
        arr: [usize;n],
    };

    let mut dp = vec![0;n];

    dp[0] = arr[0];
    dp[1] = arr[1];
    for i in 2..n{
        // 次の数と現在の数+次の次の数を比べて大きいほうをとっている
        dp[i] = dp[i-1].max(dp[i-2]+arr[i]);
    }
    // println!("{:?}",dp);
    println!("{}",dp[n-1]);
}