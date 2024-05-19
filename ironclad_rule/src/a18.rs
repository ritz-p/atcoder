use proconio::input;

fn main(){
    input!{
        n: usize,
        s: usize,
        a: [usize;n]
    };
    let mut dp = vec![vec![false;s+1];n];
    dp[0][0] = true;
    if a[0] <= s{
        dp[0][a[0]] = true;
    }
    for i in 1..n{
        for j in 0..=s{
            if dp[i-1][j]{
                dp[i][j] = true;
            }
            if j >= a[i] && dp[i-1][j-a[i]]{
                dp[i][j] = true;
            }
        }
    }

    if dp[n-1][s]{
        println!("Yes");
    }else{
        println!("No");
    }
}