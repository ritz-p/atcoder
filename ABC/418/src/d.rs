use proconio::{input, marker::Chars};
fn main() {
    input! {
        n: usize,
        t: Chars
    };
    let mut dp = vec![vec![0; 2]; n + 1];

    for i in 1..=n {
        match t[i - 1] {
            '0' => {
                dp[i][0] = dp[i - 1][1];
                dp[i][1] = dp[i - 1][0] + 1;
            }
            '1' => {
                dp[i][0] = dp[i - 1][0] + 1;
                dp[i][1] = dp[i - 1][1];
            }
            _ => {}
        }
    }
    let mut res: usize = 0;

    for i in 1..=n {
        res += dp[i][0];
    }
    println!("{}", res);
}
