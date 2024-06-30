use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize;k]
    };

    let mut dp = vec![false; n + 1];

    for i in 0..=n {
        for j in &a {
            if i >= *j && !dp[i - *j] {
                dp[i] = true;
            }
        }
    }

    if dp[n] {
        println!("First");
    } else {
        println!("Second");
    }
}
