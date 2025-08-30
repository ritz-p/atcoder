use proconio::input;
fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize,usize);n]
    };
    let mut dp = vec![vec![false; x + 1]; n + 1];
    dp[0][0] = true;
    for i in 0..n {
        let (a, b) = ab[i];
        for j in 0..x {
            if dp[i][j] {
                if j + a <= x {
                    dp[i + 1][j + a] = true;
                }
                if j + b <= x {
                    dp[i + 1][j + b] = true;
                }
            }
        }
    }
    if dp[n][x] {
        println!("Yes");
    } else {
        println!("No");
    }
}
