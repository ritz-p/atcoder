use proconio::input;
fn main() {
    input! {
        n: usize,
        m: usize,
        x: [usize;n],
        cy: [(usize,usize);m]
    };
    let mut dp = vec![vec![0; 5001]; 5001];
    let mut count = vec![0; 5001];
    let mut xs = vec![0; 5001];
    for (c, y) in cy {
        count[c] = y;
    }
    for (i, e) in x.iter().enumerate() {
        xs[i + 1] = *e;
    }

    dp[0][0] = 0;
    for i in 1..=n {
        for j in 1..=i {
            dp[i][j] = dp[i - 1][j - 1] + xs[i] + count[j];
        }
        dp[i][0] = 0;
        for j in 0..i {
            dp[i][0] = dp[i][0].max(dp[i - 1][j]);
        }
    }
    println!("{}", dp[n].iter().max().unwrap());
}
