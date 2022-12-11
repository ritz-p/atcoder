use proconio::input;
use std::cmp::min;
fn main(){
    input!{
        n: usize,
        k: usize,
        arr: [isize;n],
    };

    let mut dp = vec![std::isize::MAX;n];
    dp[0] = 0;
    for i in 1..n{
        for j in 1..=k{
            if i >= j{
                dp[i] = min(dp[i],num::abs(arr[i] - arr[i-j])+dp[i-j]);
            }
        }
        // println!("{}",dp[i]);
    }
    // println!("{:?}",dp);
    println!("{}",dp[n-1]);
}