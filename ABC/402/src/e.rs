use proconio::input;
fn main() {
    input! {
        n: usize,
        x: usize,
        scp: [(usize,usize,f64);n]
    };
    let states = 1usize << n;
    let mut dp = vec![vec![0f64; x + 1]; states];

    for m in 1..=x {
        for mask in 0..states {
            let mut best = dp[mask][m - 1];
            for (i, (s, c, p)) in scp.iter().enumerate() {
                if mask & (1 << i) != 0 || c > &m {
                    continue;
                }
                let succeed = p * (*s as f64 + dp[mask | (1 << i)][m - c]);
                let fail = (1.0 - p) * dp[mask][m - c];
                let val = succeed + fail;
                if val > best {
                    best = val;
                }
            }
            dp[mask][m] = best.max(dp[mask][m]);
        }
    }
    println!("{:.10}", dp[0][x]);
}
