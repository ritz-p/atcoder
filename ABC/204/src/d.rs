use proconio::input;

fn main() {
    input! {
        n: usize,
        t: [usize;n]
    };
    let sum = t.iter().sum::<usize>();
    let mut dp = vec![vec![usize::MAX; sum + 1]; n + 1];
    dp[0][0] = 0;
    for i in 0..n {
        for j in 0..=sum {
            if dp[i][j] == usize::MAX {
                continue;
            }
            dp[i + 1][j + t[i]] = dp[i + 1][j + t[i]].min(dp[i][j]);
            dp[i + 1][j] = dp[i + 1][j].min(dp[i][j] + t[i]);
        }
    }
    let mut res = usize::MAX;
    for i in 0..=sum {
        res = res.min(i.max(dp[n][i]));
    }
    println!("{}", res);
}
