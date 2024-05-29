use proconio::input;

fn main() {
    input! { 
        n: usize,
        m: usize,
        a: [usize; m]
    };
    let cost: [usize; 10] = [0, 2, 5, 5, 4, 5, 6, 3, 7, 6];
    let mut dp = vec![(-1, 0); n + 1];
    dp[0].0 = 0;
    for i in 1..=n {
        for &a in &a {
            if i >= cost[a] && dp[i - cost[a]].0 >= 0 {
                dp[i] = dp[i].max((dp[i - cost[a]].0 + 1, a));
            }
        }
    }
    let mut res = "".to_string();
    let mut current = n;
    while current != 0 {
        res += &dp[current].1.to_string();
        current -= cost[dp[current].1];
    }
    println!("{res}");
}
