use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        ab: [(usize,usize);n]
    };
    let mut dp = vec![vec![false; 10001]; 101];
    dp[0][0] = true;
    dp[1][ab[0].0] = true;
    dp[1][ab[0].0] = true;
    for i in 1..n + 1 {
        for j in 0..x + 1 {
            if dp[i - 1][j] {
                dp[i][j + ab[i - 1].0] = true;
                dp[i][j + ab[i - 1].1] = true;
            }
        }
    }
    if dp[n][x] {
        println!("Yes");
    } else {
        println!("No");
    }
}
