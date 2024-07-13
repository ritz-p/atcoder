use itertools::Itertools;
use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        s: Chars,
    };
    let target = "chokudai".chars().collect_vec();
    let mut dp: Vec<Vec<usize>> = vec![vec![0; 9]; s.len() + 1];
    let m = 1000000007;
    for i in 0..s.len() + 1 {
        dp[i][0] = 1;
    }

    for i in 1..s.len() + 1 {
        for j in 1..9 {
            dp[i][j] = dp[i - 1][j];
            if s[i - 1] == target[j - 1] {
                dp[i][j] += dp[i][j - 1] % m;
            }
        }
    }

    println!("{}", dp[s.len()][8] % m);
}
