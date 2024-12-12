use proconio::input;

fn main() {
    input! {
        n: usize,
        k: isize,
        a: [isize;n],
        b: [isize;n]
    };
    let mut dp = vec![false; n];
    let mut ep = vec![false; n];
    dp[0] = true;
    ep[0] = true;

    for i in 1..n {
        if dp[i - 1] {
            if (a[i] - a[i - 1]).abs() <= k {
                dp[i] = true;
            }
            if (b[i] - a[i - 1]).abs() <= k {
                ep[i] = true;
            }
        }
        if ep[i - 1] {
            if (a[i] - b[i - 1]).abs() <= k {
                dp[i] = true;
            }
            if (b[i] - b[i - 1]).abs() <= k {
                ep[i] = true;
            }
        }
    }
    if dp[n - 1] || ep[n - 1] {
        println!("Yes");
    } else {
        println!("No");
    }
}
