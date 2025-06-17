use proconio::input;
fn main() {
    input! {
        n: usize,
        h: usize,
        m: usize,
        ab: [(isize,isize);n]
    };

    let mut dp = vec![vec![-1isize; h + 1]; n + 1];

    dp[0][h] = m as isize;

    for (i, (a, b)) in ab.iter().enumerate() {
        for j in 0..=h {
            if dp[i][j] < 0 {
                continue;
            }

            if dp[i][j] >= *b {
                dp[i + 1][j] = dp[i + 1][j].max(dp[i][j] - b);
            }

            if j as isize >= *a {
                dp[i + 1][j - *a as usize] = dp[i + 1][j - *a as usize].max(dp[i][j]);
            }
        }
    }

    let mut res = 0;

    for i in 0..=n {
        for j in 0..=h {
            if dp[i][j] >= 0 {
                res = res.max(i);
            }
        }
    }

    println!("{}", res);
}
