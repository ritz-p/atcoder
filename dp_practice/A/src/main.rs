use proconio::input;
use std::cmp::min;
fn main(){
    input!{
        n: usize,
        arr: [isize;n],
    };
    let mut dp = vec![0;n];

    dp[1] = num::abs(arr[1] - arr[0]);
    for i in 2..n{
        let distance1 = num::abs(arr[i] - arr[i-1]) + dp[i-1];
        let distance2 = num::abs(arr[i] - arr[i-2]) + dp[i-2];
        dp[i] = min(distance1,distance2);
        // println!("{},{}",distance1,distance2);
    }
    println!("{}",dp[n-1]);
}