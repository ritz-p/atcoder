use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: Chars
    };
    let mut dp = vec![vec![0; 3]; n + 1];
    let mut hands = vec![];
    for c in &s {
        match c {
            'R' => {
                hands.push(0);
            }
            'P' => {
                hands.push(1);
            }
            'S' => {
                hands.push(2);
            }
            _ => {}
        }
    }
    for i in 0..n {
        for j in 0..3 {
            for k in 0..3 {
                if j != k {
                    if (hands[i] + 1) % 3 == k as u8 {
                        dp[i + 1][k] = dp[i + 1][k].max(dp[i][j] + 1);
                    } else if hands[i] == k as u8 {
                        dp[i + 1][k] = dp[i + 1][k].max(dp[i][j]);
                    }
                }
            }
        }
    }
    println!("{}", dp[n].iter().max().unwrap());
}
