use itertools::Itertools;
use proconio::{input, marker::Chars};
fn main() {
    input! {
        t: usize,
    };
    let mut res = vec![];
    for _ in 0..t {
        input! {
            n: usize,
            s: Chars
        };
        let mut dp = vec![false; 1 << n];
        dp[0] = true;
        let m = 1 << n;

        for b in 1..m {
            if s[b - 1] == '1' {
                continue;
            }

            for i in 0..n {
                if (b & (1 << i)) != 0 && dp[b ^ (1 << i)] {
                    dp[b] = true;
                    break;
                }
            }
        }
        if dp[m - 1] {
            res.push("Yes");
        } else {
            res.push("No");
        }
    }
    println!("{}", res.iter().join("\n"));
}
