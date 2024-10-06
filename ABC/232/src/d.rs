use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: usize,
        w: usize,
        c: [Chars;h]
    };

    let mut dp = vec![vec![0; w]; h];
    dp[0][0] = 1;
    let mut res = 1;

    for i in 0..h {
        for j in 0..w {
            if c[i][j] == '#' {
                continue;
            }
            if dp[i][j] == 0 {
                continue;
            }
            if i < h - 1 && c[i + 1][j] == '.' {
                dp[i + 1][j] = dp[i + 1][j].max(dp[i][j] + 1);
            }
            if j < w - 1 && c[i][j + 1] == '.' {
                dp[i][j + 1] = dp[i][j + 1].max(dp[i][j] + 1);
            }
        }
    }

    for i in 0..h {
        for j in 0..w {
            res = res.max(dp[i][j]);
        }
    }
    println!("{}", res);
}
