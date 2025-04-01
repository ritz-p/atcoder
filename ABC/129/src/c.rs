use std::{collections::HashSet, iter::FromIterator};

use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        a: [usize;m]
    };
    let modulo = 1000000007;
    let mut dp = vec![0; n + 1];
    let set: HashSet<usize> = HashSet::from_iter(a.iter().cloned());
    if n >= 1 && !set.contains(&1) {
        dp[1] += 1;
    }
    if n >= 2 && !set.contains(&2) {
        dp[2] += 1;
    }
    for i in 1..=n {
        if i < n {
            if !set.contains(&(i + 1)) {
                dp[i + 1] += dp[i];
                dp[i + 1] %= modulo;
            }
        }

        if i < n - 1 {
            if !set.contains(&(i + 2)) {
                dp[i + 2] += dp[i];
                dp[i + 2] %= modulo;
            }
        }
    }
    println!("{}", dp[n]);
}
