use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        s: usize,
        a: [usize;n]
    };
    let mut dp = vec![vec![false; s + 1]; n];
    dp[0][0] = true;
    if a[0] <= s {
        dp[0][a[0]] = true;
    }
    for i in 1..n {
        for j in 0..=s {
            if dp[i - 1][j] {
                dp[i][j] = true;
            }
            if j >= a[i] && dp[i - 1][j - a[i]] {
                dp[i][j] = true;
            }
        }
    }
    if !dp[n - 1][s] {
        println!("-1");
        return;
    }
    let mut res = vec![];
    let mut current = s;
    let mut index = n - 1;
    loop {
        if current == 0 {
            break;
        }
        if index == 0 {
            res.push(index);
            break;
        }
        if current >= a[index] && dp[index - 1][current - a[index]] {
            res.push(index);
            current -= a[index];
        }
        index -= 1;
    }

    println!("{}", res.len());
    println!("{}", res.iter().rev().map(|v| v + 1).join(" "));
}
