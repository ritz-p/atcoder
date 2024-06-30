use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize;n-1]
    };

    let mut graph = vec![vec![]; n + 1];
    for i in 0..n - 1 {
        graph[a[i]].push(i + 2);
    }
    let mut dp = vec![0; n + 1];
    for i in (1..=n).rev() {
        for g in &graph[i] {
            dp[i] += dp[*g] + 1;
        }
    }
    for i in 1..=n {
        print!("{} ", dp[i]);
    }
}
