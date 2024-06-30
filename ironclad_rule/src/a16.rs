use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize;n-1],
        mut b: [usize;n-2],
    };
    a.insert(0, 0);
    b.insert(0, 0);
    b.insert(0, 0);
    let mut dp = vec![0; n + 1];
    dp[0] = 0;
    dp[1] = a[1];
    for i in 2..n {
        dp[i] += (dp[i - 1] + a[i]).min(dp[i - 2] + b[i]);
    }
    println!("{}", dp[n - 1]);
}
