use proconio::input;
use proconio::marker::Chars;
fn main() {
    input! {
        s: Chars,
        t: Chars,
    };

    let mut dp = vec![vec![0; 2009]; 2009];
    let n = s.len();
    let m = t.len();
    for i in 0..=n {
        for j in 0..=m {
            if i > 0 && j > 0 && s[i - 1] == t[j - 1] {
                //i-1 = 削除,j-1 = 挿入 i-1,j-1 変更
                dp[i][j] = (dp[i - 1][j] + 1)
                    .min(dp[i][j - 1] + 1)
                    .min(dp[i - 1][j - 1]);
            } else if i > 0 && j > 0 {
                dp[i][j] = (dp[i - 1][j] + 1)
                    .min(dp[i][j - 1] + 1)
                    .min(dp[i - 1][j - 1] + 1);
            } else if i > 0 {
                dp[i][j] = dp[i - 1][j] + 1;
            } else if j > 0 {
                dp[i][j] = dp[i][j - 1] + 1;
            }
        }
    }

    println!("{}", dp[n][m]);
}
