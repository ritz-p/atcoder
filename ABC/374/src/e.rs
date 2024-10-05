use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        apbq: [(usize,usize,usize,usize);n]
    };
    let mut left = 0;
    let mut right = 10000000;

    while left < right {
        let mid = (left + right + 1) / 2;
        if dp(n, x, &apbq, mid) {
            left = mid;
        } else {
            right = mid - 1;
        }
    }

    println!("{}", left);
}

fn dp(n: usize, x: usize, apbq: &Vec<(usize, usize, usize, usize)>, m: usize) -> bool {
    let mut dp = vec![x + 1; x + 1];
    dp[0] = 0;

    for i in 0..n {
        let mut next = dp.clone();
        let (a, p, b, q) = apbq[i];
        let cs = if a >= m { 0 } else { (m + a - 1) / a * p };
        let ct = if b >= m { 0 } else { (m + b - 1) / b * q };

        for j in 0..=x {
            if dp[j] < x + 1 {
                if j + cs <= x {
                    next[j + cs] = next[j + cs].min(dp[j]);
                }
                if j + ct <= x {
                    next[j + ct] = next[j + ct].min(dp[j]);
                }
            }
        }

        dp = next;
    }

    dp.iter().any(|&c| c <= x)
}
