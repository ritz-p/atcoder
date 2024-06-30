use itertools::Itertools;
use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize;n-1],
        mut b: [usize;n-2],
    };
    a.insert(0, 0);
    b.insert(0, 0);
    b.insert(0, 0);
    let mut dp = vec![0; n];
    dp[0] = 0;
    dp[1] = a[1];

    for i in 2..n {
        dp[i] = (dp[i - 1] + a[i]).min(dp[i - 2] + b[i]);
    }
    let mut p = n - 1;
    let mut res = vec![];
    res.push(n - 1);
    loop {
        if p == 0 {
            break;
        }
        if dp[p] == dp[p - 1] + a[p] {
            p -= 1;
        } else {
            p -= 2;
        }
        res.push(p);
    }
    println!("{}", res.len());
    println!("{}", res.iter().rev().map(|v| v + 1).join(" "));
}
