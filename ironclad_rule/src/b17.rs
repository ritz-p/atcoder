use itertools::Itertools;
use proconio::input;

fn main(){
    input!{
        n: usize,
        h: [isize;n]
    }
    let mut dp = vec![0;n];
    
    dp[1] = (h[0] - h[1]).abs();

    for i in 2..n{
        dp[i] = (dp[i-1] + (h[i]-h[i-1]).abs()).min(dp[i-2] + (h[i]-h[i-2]).abs());
    }

    let mut p = n-1;
    let mut res = vec![];
    res.push(p+1);

    loop{
        if p == 0{
            break;
        }
        if dp[p] == dp[p-1] + (h[p] - h[p-1]).abs(){
            p -= 1;
        }else{
            p -= 2;
        }
        res.push(p+1);
    }
    println!("{}",res.len());
    println!("{}",res.iter().rev().map(|e|e.to_string()).join(" "));
}